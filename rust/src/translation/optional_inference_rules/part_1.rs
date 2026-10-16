// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::MlOptionalFactFieldLoop1Result;
use super::MlPlanInferenceTermIdentityJoin8;
use super::MlPlanInferenceTermIdentityJoin9;
use super::MlPlanInferenceTermIdentityLoop6Result;
use super::MlPlanInferenceTermIdentityLoop7Result;
use super::MlPlanInferenceTermIdentityLoop10Result;
use super::ml_optional_fact_field_loop1;
use super::ml_plan_inference_term_identity_loop6;
use super::ml_plan_inference_term_identity_loop7;
use super::ml_plan_inference_term_identity_loop10;
use super::ml_plan_inference_term_identity_loop14;
use super::ml_plan_optional_additional_absence_loop2;
use super::ml_plan_optional_additional_absence_loop3;
use super::read_required_text_fact;

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
pub fn optional_fact_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    field: &str,
) -> f64 {
    if (node < 0f64) || (node >= (starts.len() as f64)) {
        -1f64
    } else {
        {
            let at = self::read_required_number_fact(starts, node);
            {
                let end = at + self::read_required_number_fact(counts, node);
                {
                    let ml_s1 = self::ml_optional_fact_field_loop1(labels, targets, field, at, end);
                    match ml_s1 {
                        self::MlOptionalFactFieldLoop1Result::MlOptionalFactFieldLoop1Done => -1f64,
                        self::MlOptionalFactFieldLoop1Result::MlOptionalFactFieldLoop1Return(
                            ml_result1,
                        ) => ml_result1,
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn optional_fact_text(texts: &[String], node: f64) -> String {
    if (node < 0f64) || (node >= (texts.len() as f64)) {
        String::new()
    } else {
        self::read_required_text_fact(texts, node)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn optional_fact_tag(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    field: &str,
) -> String {
    self::optional_fact_text(
        texts,
        self::optional_fact_field(starts, counts, labels, targets, node, field),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn optional_fact_element(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    node: f64,
    index: f64,
) -> f64 {
    if ((node < 0f64) || (index < 0f64))
        || (index >= (self::read_required_number_fact(counts, node)))
    {
        -1f64
    } else {
        self::read_required_number_fact(
            targets,
            (self::read_required_number_fact(starts, node) + index),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_optional_type_constraints(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    left: f64,
    right: f64,
) -> Vec<f64> {
    {
        let left_optional =
            (self::optional_fact_tag(texts, starts, counts, labels, targets, left, "kind"))
                == "option";
        {
            let right_optional =
                (self::optional_fact_tag(texts, starts, counts, labels, targets, right, "kind"))
                    == "option";
            if ((left_optional) || (right_optional)) {
                if left_optional {
                    {
                        let first = self::optional_fact_field(
                            starts, counts, labels, targets, left, "element",
                        );
                        {
                            let second = if right_optional {
                                self::optional_fact_field(
                                    starts, counts, labels, targets, right, "element",
                                )
                            } else {
                                right
                            };
                            vec![
                                first,
                                second,
                                if left_optional && right_optional {
                                    left
                                } else {
                                    -1f64
                                },
                                if left_optional && right_optional {
                                    right
                                } else {
                                    -1f64
                                },
                            ]
                        }
                    }
                } else {
                    vec![
                        self::optional_fact_field(
                            starts, counts, labels, targets, right, "element",
                        ),
                        left,
                        -1f64,
                        -1f64,
                    ]
                }
            } else {
                Vec::<f64>::new()
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
pub fn plan_optional_coalescing_constraints(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    left: f64,
    resolved: f64,
    right: f64,
    force: bool,
) -> Vec<f64> {
    if force {
        vec![1f64, left, right]
    } else {
        {
            let kind =
                self::optional_fact_tag(texts, starts, counts, labels, targets, resolved, "kind");
            if kind == "var" {
                Vec::<f64>::new()
            } else {
                vec![
                    0f64,
                    if kind == "option" {
                        self::optional_fact_field(
                            starts, counts, labels, targets, resolved, "element",
                        )
                    } else {
                        left
                    },
                    right,
                ]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_optional_field_inference(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    object: f64,
    resolved: f64,
) -> Vec<f64> {
    {
        let kind =
            self::optional_fact_tag(texts, starts, counts, labels, targets, resolved, "kind");
        if kind == "var" {
            vec![-1f64, object]
        } else {
            vec![if kind == "option" {
                self::optional_fact_field(starts, counts, labels, targets, resolved, "element")
            } else {
                object
            }]
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
pub fn plan_optional_join_constraints(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    left: f64,
    right: f64,
    resolved_left: f64,
    resolved_right: f64,
    current: f64,
) -> Vec<f64> {
    {
        let left_kind = self::optional_fact_tag(
            texts,
            starts,
            counts,
            labels,
            targets,
            resolved_left,
            "kind",
        );
        {
            let right_kind = self::optional_fact_tag(
                texts,
                starts,
                counts,
                labels,
                targets,
                resolved_right,
                "kind",
            );
            if (left_kind == "unit") == (right_kind == "unit") {
                vec![
                    0f64,
                    if right_kind == "option" { right } else { left },
                    left,
                    right,
                ]
            } else {
                {
                    let element = if left_kind == "unit" { right } else { left };
                    if (((self::optional_fact_tag(
                        texts, starts, counts, labels, targets, current, "kind",
                    )) == "option")
                        && ((self::optional_fact_field(
                            starts, counts, labels, targets, current, "element",
                        )) == element))
                        && ((self::optional_fact_tag(
                            texts, starts, counts, labels, targets, current, "absence",
                        )) == "undefined")
                    {
                        vec![1f64, current]
                    } else {
                        vec![2f64, element]
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_optional_additional_absence(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    actual: f64,
    declared: f64,
) -> Vec<f64> {
    {
        let actual_kind =
            self::optional_fact_tag(texts, starts, counts, labels, targets, actual, "kind");
        {
            let declared_kind =
                self::optional_fact_tag(texts, starts, counts, labels, targets, declared, "kind");
            if actual_kind == "option" {
                {
                    let policy = self::optional_fact_tag(
                        texts, starts, counts, labels, targets, declared, "absence",
                    );
                    if (declared_kind != "option")
                        || (((!(policy).is_empty()) && (policy != "both"))
                            && ((self::optional_fact_tag(
                                texts, starts, counts, labels, targets, actual, "absence",
                            )) != policy))
                    {
                        vec![1f64]
                    } else {
                        vec![
                            0f64,
                            self::optional_fact_field(
                                starts, counts, labels, targets, actual, "element",
                            ),
                            self::optional_fact_field(
                                starts, counts, labels, targets, declared, "element",
                            ),
                        ]
                    }
                }
            } else if actual_kind == declared_kind {
                if ((actual_kind == "array") || (actual_kind == "set")) || (actual_kind == "cell") {
                    vec![
                        0f64,
                        self::optional_fact_field(
                            starts, counts, labels, targets, actual, "element",
                        ),
                        self::optional_fact_field(
                            starts, counts, labels, targets, declared, "element",
                        ),
                    ]
                } else if actual_kind == "map" {
                    vec![
                        0f64,
                        self::optional_fact_field(starts, counts, labels, targets, actual, "key"),
                        self::optional_fact_field(starts, counts, labels, targets, declared, "key"),
                        self::optional_fact_field(starts, counts, labels, targets, actual, "value"),
                        self::optional_fact_field(
                            starts, counts, labels, targets, declared, "value",
                        ),
                    ]
                } else if actual_kind == "function" {
                    vec![
                        0f64,
                        self::optional_fact_field(starts, counts, labels, targets, actual, "ret"),
                        self::optional_fact_field(starts, counts, labels, targets, declared, "ret"),
                    ]
                } else {
                    {
                        let pairs = vec![0f64];
                        {
                            let pairs_3 = if actual_kind == "tuple" {
                                {
                                    let values = self::optional_fact_field(
                                        starts, counts, labels, targets, actual, "elements",
                                    );
                                    {
                                        let expected = self::optional_fact_field(
                                            starts, counts, labels, targets, declared, "elements",
                                        );
                                        {
                                            let at = 0f64;
                                            {
                                                self::ml_plan_optional_additional_absence_loop2(
                                                    texts, starts, counts, labels, targets, pairs,
                                                    values, expected, at,
                                                )
                                            }
                                        }
                                    }
                                }
                            } else {
                                pairs
                            };
                            {
                                if actual_kind == "record" {
                                    {
                                        let fields = self::optional_fact_field(
                                            starts, counts, labels, targets, actual, "fields",
                                        );
                                        {
                                            let expected_2 = self::optional_fact_field(
                                                starts, counts, labels, targets, declared, "fields",
                                            );
                                            {
                                                let at_2 = 0f64;
                                                {
                                                    self::ml_plan_optional_additional_absence_loop3(
                                                        texts, starts, counts, labels, targets,
                                                        pairs_3, fields, expected_2, at_2,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    pairs_3
                                }
                            }
                        }
                    }
                }
            } else {
                vec![0f64]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_inference_term_identity(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    left: f64,
    right: f64,
) -> Vec<f64> {
    {
        let kind = self::optional_fact_tag(texts, starts, counts, labels, targets, left, "kind");
        if ((((kind
            != (self::optional_fact_tag(texts, starts, counts, labels, targets, right, "kind")))
            || ((kind == "var")
                && ((self::optional_fact_tag(
                    texts, starts, counts, labels, targets, left, "id",
                )) != (self::optional_fact_tag(
                    texts, starts, counts, labels, targets, right, "id",
                )))))
            || ((kind == "fixed")
                && (((self::optional_fact_tag(
                    texts, starts, counts, labels, targets, left, "bits",
                )) != (self::optional_fact_tag(
                    texts, starts, counts, labels, targets, right, "bits",
                ))) || ((self::optional_fact_tag(
                    texts, starts, counts, labels, targets, left, "signed",
                )) != (self::optional_fact_tag(
                    texts, starts, counts, labels, targets, right, "signed",
                ))))))
            || ((kind == "data")
                && ((self::optional_fact_tag(
                    texts, starts, counts, labels, targets, left, "name",
                )) != (self::optional_fact_tag(
                    texts, starts, counts, labels, targets, right, "name",
                )))))
            || (((((kind == "array") || (kind == "record")) || (kind == "map")) || (kind == "set"))
                && (((self::optional_fact_tag(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    left,
                    "reference",
                )) == "true")
                    != ((self::optional_fact_tag(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        right,
                        "reference",
                    )) == "true")))
        {
            Vec::<f64>::new()
        } else if kind == "option" {
            {
                let first = left;
                {
                    let second = right;
                    {
                        let first_absence = self::optional_fact_tag(
                            texts, starts, counts, labels, targets, first, "absence",
                        );
                        {
                            let second_absence = self::optional_fact_tag(
                                texts, starts, counts, labels, targets, second, "absence",
                            );
                            {
                                let first_absence_3 = if (first_absence).is_empty() {
                                    { String::from("both") }
                                } else {
                                    first_absence
                                };
                                {
                                    let second_absence_3 = if (second_absence).is_empty() {
                                        { String::from("both") }
                                    } else {
                                        second_absence
                                    };
                                    {
                                        let first_2 = self::optional_fact_field(
                                            starts, counts, labels, targets, first, "element",
                                        );
                                        {
                                            let second_2 = self::optional_fact_field(
                                                starts, counts, labels, targets, second, "element",
                                            );
                                            {
                                                let ml_s2 =
                                                    self::ml_plan_inference_term_identity_loop6(
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        first_2,
                                                        first_absence_3,
                                                    );
                                                match ml_s2 {
                                                                        self::MlPlanInferenceTermIdentityLoop6Result::MlPlanInferenceTermIdentityLoop6Done(first_3, first_absence_4) => {
                                                                            {
                                                                                let ml_s3 = self::ml_plan_inference_term_identity_loop7(texts, starts, counts, labels, targets, second_2, second_absence_3);
                                                                                match ml_s3 {
                                                                                    self::MlPlanInferenceTermIdentityLoop7Result::MlPlanInferenceTermIdentityLoop7Done(second_3, second_absence_4) => {
                                                                                        if first_absence_4 == second_absence_4 {
                                                                                            vec![1f64, first_3, second_3]
                                                                                        } else {
                                                                                            Vec::<f64>::new()
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
        } else if ((kind == "array") || (kind == "set")) || (kind == "cell") {
            vec![
                1f64,
                self::optional_fact_field(starts, counts, labels, targets, left, "element"),
                self::optional_fact_field(starts, counts, labels, targets, right, "element"),
            ]
        } else if kind == "map" {
            vec![
                1f64,
                self::optional_fact_field(starts, counts, labels, targets, left, "key"),
                self::optional_fact_field(starts, counts, labels, targets, right, "key"),
                self::optional_fact_field(starts, counts, labels, targets, left, "value"),
                self::optional_fact_field(starts, counts, labels, targets, right, "value"),
            ]
        } else {
            {
                let pairs = vec![1f64];
                {
                    let ml_s4 = if ((kind == "tuple") || (kind == "union")) || (kind == "function")
                    {
                        {
                            let field = if kind == "tuple" {
                                String::from("elements")
                            } else if kind == "union" {
                                String::from("alternatives")
                            } else {
                                String::from("params")
                            };
                            {
                                let first_4 = self::optional_fact_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    left,
                                    &(field),
                                );
                                {
                                    let second_4 = self::optional_fact_field(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        right,
                                        &(field),
                                    );
                                    {
                                        let length =
                                            self::read_required_number_fact(counts, first_4);
                                        {
                                            let ml_library_inequality_3 = length
                                                != (self::read_required_number_fact(
                                                    counts, second_4,
                                                ));
                                            if ml_library_inequality_3 {
                                                self::MlPlanInferenceTermIdentityJoin8::MlPlanInferenceTermIdentityJoin8Return(Vec::<f64>::new())
                                            } else {
                                                {
                                                    let at = 0f64;
                                                    {
                                                        let pairs_2 = self::ml_plan_inference_term_identity_loop14(starts, counts, targets, pairs, first_4, second_4, length, at);
                                                        if kind == "function" {
                                                            {
                                                                let pairs_3 = {
                                                                    let (ml_append_right, ml_append_left) = (vec![self::optional_fact_field(starts, counts, labels, targets, left, "ret"), self::optional_fact_field(starts, counts, labels, targets, right, "ret")], pairs_2);
                                                                    self::ml_array::append(
                                                                        ml_append_left,
                                                                        ml_append_right,
                                                                    )
                                                                };
                                                                self::MlPlanInferenceTermIdentityJoin8::MlPlanInferenceTermIdentityJoin8Next(pairs_3)
                                                            }
                                                        } else {
                                                            self::MlPlanInferenceTermIdentityJoin8::MlPlanInferenceTermIdentityJoin8Next(pairs_2)
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
                        self::MlPlanInferenceTermIdentityJoin8::MlPlanInferenceTermIdentityJoin8Next(
                            pairs,
                        )
                    };
                    match ml_s4 {
                                                    self::MlPlanInferenceTermIdentityJoin8::MlPlanInferenceTermIdentityJoin8Next(pairs_4) => {
                                                        {
                                                            let ml_s6 = if kind == "record" {
                                                                {
let ml_library_inequality_2 = ((self::optional_fact_tag(texts, starts, counts, labels, targets, left, "open")) == "true") != ((self::optional_fact_tag(texts, starts, counts, labels, targets, right, "open")) == "true") ;
if ml_library_inequality_2 {
                                                                    self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Return(Vec::<f64>::new())
                                                                } else {
                                                                    {
                                                                        let first_5 = self::optional_fact_field(starts, counts, labels, targets, left, "fields");
                                                                        {
                                                                            let second_5 = self::optional_fact_field(starts, counts, labels, targets, right, "fields");
                                                                            {
                                                                                let length_2 = self::read_required_number_fact(counts, first_5);
                                                                                {
let ml_library_inequality_1 = length_2 != (self::read_required_number_fact(counts, second_5)) ;
if ml_library_inequality_1 {
                                                                                    self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Return(Vec::<f64>::new())
                                                                                } else {
                                                                                    {
                                                                                        let at_2 = 0f64;
                                                                                        {
                                                                                            let ml_s7 = self::ml_plan_inference_term_identity_loop10(texts, starts, counts, labels, targets, pairs_4, first_5, second_5, length_2, at_2);
                                                                                            match ml_s7 {
                                                                                                self::MlPlanInferenceTermIdentityLoop10Result::MlPlanInferenceTermIdentityLoop10Done(pairs_5) => {
                                                                                                    self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Next(pairs_5)
                                                                                                }
                                                                                                self::MlPlanInferenceTermIdentityLoop10Result::MlPlanInferenceTermIdentityLoop10Return(ml_result10) => {
                                                                                                    self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Return(ml_result10)
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
                                                                self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Next(pairs_4)
                                                            };
                                                            match ml_s6 {
                                                                self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Next(pairs_6) => {
                                                                    pairs_6
                                                                }
                                                                self::MlPlanInferenceTermIdentityJoin9::MlPlanInferenceTermIdentityJoin9Return(ml_result9) => {
                                                                    ml_result9
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlPlanInferenceTermIdentityJoin8::MlPlanInferenceTermIdentityJoin8Return(ml_result13) => {
                                                        ml_result13
                                                    }
                                                }
                }
            }
        }
    }
}

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
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Number fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}
