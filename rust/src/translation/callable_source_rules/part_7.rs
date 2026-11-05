// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::callable_bound;
use super::callable_element;
use super::callable_field;
use super::callable_nested_events;
use super::callable_outer_assignments;
use super::callable_statement_uses;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::read_callable_statement_uses;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_read_callable_statement_uses_loop29;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_statement_uses_loop28(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut statements: f64,
    mut events: Vec<f64>,
    mut scope: Vec<String>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (statements >= 0f64) && (at < (self::read_required_number_fact(counts, statements))) {
            {
                let statement = self::callable_element(starts, counts, targets, statements, at);
                {
                    let next = self::callable_statement_uses(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        statement,
                        &(scope),
                    );
                    {
                        let index = 0f64;
                        {
                            let scope_2 = self::source_ml_source1_temporary_read_callable_statement_uses_loop29(texts, scope, &(next), index);
                            {
                                let events_2 = {
                                    let source_ml_source2_concat_value151 = events;
                                    {
                                        let source_ml_source2_concat_value152 = next;
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value151,
                                            ),
                                            source_ml_source2_concat_value152,
                                        )
                                    }
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (events, scope, at) = (events_2, scope_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { events };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_nested_events_loop30(
    events: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (events.len() as f64) {
            {
                let result_3 = if (self::read_required_number_fact(events, at)) == 2f64 {
                    result
                } else {
                    {
                        {
                            let source_ml_source2_concat_value153 = result;
                            {
                                let source_ml_source2_concat_value154 = vec![
                                    self::read_required_number_fact(events, at),
                                    self::read_required_number_fact(events, (at + 1f64)),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value153,
                                    ),
                                    source_ml_source2_concat_value154,
                                )
                            }
                        }
                    }
                };
                {
                    let at_2 = at + 2f64;
                    {
                        (result, at) = (result_3, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_statement_uses_loop31(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bound: &[String],
    mut events: Vec<f64>,
    mut clauses: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (clauses >= 0f64) && (at < (self::read_required_number_fact(counts, clauses))) {
            {
                let clause = self::callable_element(starts, counts, targets, clauses, at);
                {
                    let events_2 = {
                        let source_ml_source2_concat_value155 = events;
                        {
                            let source_ml_source2_concat_value156 = self::callable_nested_events(
                                &(self::read_callable_statement_uses(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::callable_field(
                                        starts, counts, labels, targets, clause, "body",
                                    ),
                                    bound,
                                )),
                            );
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value155,
                                ),
                                source_ml_source2_concat_value156,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (events, at) = (events_2, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { events };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_statement_uses_loop33(
    texts: &[String],
    mut inner: Vec<String>,
    mut at: f64,
    update: &[f64],
) -> Vec<String> {
    loop {
        if at < (update.len() as f64) {
            {
                let inner_3 = if (self::read_required_number_fact(update, at)) == 2f64 {
                    {
                        {
                            let source_ml_source2_concat_value157 = inner;
                            {
                                let source_ml_source2_concat_value158 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(update, (at + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value157,
                                    ),
                                    source_ml_source2_concat_value158,
                                )
                            }
                        }
                    }
                } else {
                    inner
                };
                {
                    let at_2 = at + 2f64;
                    {
                        (inner, at) = (inner_3, at_2);
                        continue;
                    }
                }
            }
        }
        return { inner };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableStatementUsesLoop32Result {
    MlSource1TemporaryCallableStatementUsesLoop32Done(Vec<String>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_statement_uses_loop32(
    texts: &[String],
    initial: &[f64],
    mut inner: Vec<String>,
    mut at: f64,
) -> self::MlSource1TemporaryCallableStatementUsesLoop32Result {
    loop {
        if at < (initial.len() as f64) {
            {
                let inner_3 = if (self::read_required_number_fact(initial, at)) == 2f64 {
                    {
                        {
                            let source_ml_source2_concat_value159 = inner;
                            {
                                let source_ml_source2_concat_value160 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(initial, (at + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value159,
                                    ),
                                    source_ml_source2_concat_value160,
                                )
                            }
                        }
                    }
                } else {
                    inner
                };
                {
                    let at_2 = at + 2f64;
                    {
                        (inner, at) = (inner_3, at_2);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryCallableStatementUsesLoop32Result::MlSource1TemporaryCallableStatementUsesLoop32Done(inner, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadCallableCaptureConflictsJoin37 {
    MlSource1TemporaryReadCallableCaptureConflictsJoin37Next(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_capture_conflicts_loop36(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    assigned: &[String],
    closures: &[f64],
    mut at: f64,
    mut result: Vec<f64>,
    mut captures: f64,
    mut index: f64,
    mut found: bool,
) -> Vec<f64> {
    loop {
        return if ((captures >= 0f64)
            && (index < (self::read_required_number_fact(counts, captures))))
            && !found
        {
            {
                let capture = self::callable_element(starts, counts, targets, captures, index);
                {
                    let source_ml_source2_s163 = if self::callable_bound(
                        assigned,
                        &(self::callable_text(texts, capture)),
                    ) {
                        {
                            let result_2 = {
                                let source_ml_source2_concat_value164 = result;
                                {
                                    let source_ml_source2_concat_value165 = vec![
                                        self::read_required_number_fact(closures, at),
                                        capture,
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value164,
                                        ),
                                        source_ml_source2_concat_value165,
                                    )
                                }
                            };
                            {
                                let found_2 = true;
                                self::MlSource1TemporaryReadCallableCaptureConflictsJoin37::MlSource1TemporaryReadCallableCaptureConflictsJoin37Next(result_2, found_2)
                            }
                        }
                    } else {
                        self::MlSource1TemporaryReadCallableCaptureConflictsJoin37::MlSource1TemporaryReadCallableCaptureConflictsJoin37Next(result, found)
                    };
                    match source_ml_source2_s163 {
                        self::MlSource1TemporaryReadCallableCaptureConflictsJoin37::MlSource1TemporaryReadCallableCaptureConflictsJoin37Next(result_3, found_3) => {
                            {
                                let index_2 = index + 1f64;
                                {
                                    (result, index, found) = (result_3, index_2, found_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
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
pub fn source_ml_source1_temporary_read_callable_capture_conflicts_loop35(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    assigned: &[String],
    closures: &[f64],
    mut at: f64,
    mut result: Vec<f64>,
) -> Vec<f64> {
    loop {
        if at < (closures.len() as f64) {
            {
                let captures = self::callable_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    self::read_required_number_fact(closures, at),
                    "captures",
                );
                {
                    let index = 0f64;
                    {
                        let found = false;
                        {
                            let result_2 = self::source_ml_source1_temporary_read_callable_capture_conflicts_loop36(texts, starts, counts, targets, assigned, closures, at, result, captures, index, found);
                            {
                                let at_2 = at + 1f64;
                                {
                                    (at, result) = (at_2, result_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadCallableCaptureConflictsLoop34Result {
    MlSource1TemporaryReadCallableCaptureConflictsLoop34Done(Vec<String>, Vec<f64>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadCallableCaptureConflictsJoin38 {
    MlSource1TemporaryReadCallableCaptureConflictsJoin38Next(Vec<String>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_capture_conflicts_loop34(
    texts: &[String],
    observations: &[f64],
    mut assigned: Vec<String>,
    mut closures: Vec<f64>,
    mut at: f64,
) -> self::MlSource1TemporaryReadCallableCaptureConflictsLoop34Result {
    loop {
        return if at < (observations.len() as f64) {
            {
                let source_ml_source2_s170 = if (self::read_required_number_fact(observations, at))
                    == 0f64
                {
                    {
                        let assigned_2 = {
                            let source_ml_source2_concat_value171 = assigned;
                            {
                                let source_ml_source2_concat_value172 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(observations, (at + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value171,
                                    ),
                                    source_ml_source2_concat_value172,
                                )
                            }
                        };
                        self::MlSource1TemporaryReadCallableCaptureConflictsJoin38::MlSource1TemporaryReadCallableCaptureConflictsJoin38Next(assigned_2, closures)
                    }
                } else {
                    {
                        let closures_2 = {
                            let source_ml_source2_concat_value173 = closures;
                            {
                                let source_ml_source2_concat_value174 =
                                    vec![self::read_required_number_fact(
                                        observations,
                                        (at + 1f64),
                                    )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value173,
                                    ),
                                    source_ml_source2_concat_value174,
                                )
                            }
                        };
                        self::MlSource1TemporaryReadCallableCaptureConflictsJoin38::MlSource1TemporaryReadCallableCaptureConflictsJoin38Next(assigned, closures_2)
                    }
                };
                match source_ml_source2_s170 {
                    self::MlSource1TemporaryReadCallableCaptureConflictsJoin38::MlSource1TemporaryReadCallableCaptureConflictsJoin38Next(assigned_3, closures_3) => {
                        {
                            let at_2 = at + 2f64;
                            {
                                (assigned, closures, at) = (assigned_3, closures_3, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadCallableCaptureConflictsLoop34Result::MlSource1TemporaryReadCallableCaptureConflictsLoop34Done(assigned, closures, at)
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
pub fn source_ml_source1_temporary_callable_outer_assignments_loop39(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            {
                let result_3 = if ((self::read_required_text_fact(labels, at)) != "span")
                    && ((self::read_required_text_fact(labels, at)) != "type")
                {
                    {
                        {
                            let source_ml_source2_concat_value175 = result;
                            {
                                let source_ml_source2_concat_value176 =
                                    self::callable_outer_assignments(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        self::read_required_number_fact(targets, at),
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value175,
                                    ),
                                    source_ml_source2_concat_value176,
                                )
                            }
                        }
                    }
                } else {
                    result
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_3, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_callable_parameter_types_loop40(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut parameters: f64,
    expected_kind: &str,
    mut expected_params: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (parameters >= 0f64) && (at < (self::read_required_number_fact(counts, parameters))) {
            {
                let parameter = self::callable_element(starts, counts, targets, parameters, at);
                {
                    let explicit =
                        self::callable_field(starts, counts, labels, targets, parameter, "type");
                    {
                        let type_ = if self::callable_truthy(forms, texts, explicit) {
                            explicit
                        } else if expected_kind == "function" {
                            self::callable_element(starts, counts, targets, expected_params, at)
                        } else {
                            -1f64
                        };
                        {
                            let result_2 = {
                                let source_ml_source2_concat_value177 = result;
                                {
                                    let source_ml_source2_concat_value178 = vec![
                                        parameter,
                                        type_,
                                        if self::callable_truthy(forms, texts, explicit) {
                                            1f64
                                        } else {
                                            0f64
                                        },
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value177,
                                        ),
                                        source_ml_source2_concat_value178,
                                    )
                                }
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_function_type_loop41(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut parameters: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (parameters >= 0f64) && (at < (self::read_required_number_fact(counts, parameters))) {
            {
                let parameter = self::callable_element(starts, counts, targets, parameters, at);
                {
                    let plan_2 = {
                        let source_ml_source2_concat_value179 = plan;
                        {
                            let source_ml_source2_concat_value180 = vec![self::callable_field(
                                starts, counts, labels, targets, parameter, "type",
                            )];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value179,
                                ),
                                source_ml_source2_concat_value180,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (plan, at) = (plan_2, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_type_constraints_loop42(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut left_params: f64,
    mut right_params: f64,
    mut pairs: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::read_required_number_fact(counts, left_params)) {
            {
                let pairs_2 = {
                    let source_ml_source2_concat_value181 = pairs;
                    {
                        let source_ml_source2_concat_value182 = vec![
                            self::callable_element(starts, counts, targets, left_params, at),
                            self::callable_element(starts, counts, targets, right_params, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value181,
                            ),
                            source_ml_source2_concat_value182,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_application_constraints_loop43(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut arguments_node: f64,
    mut params: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::read_required_number_fact(counts, params)) {
            {
                let plan_2 = {
                    let source_ml_source2_concat_value183 = plan;
                    {
                        let source_ml_source2_concat_value184 = vec![
                            self::callable_element(starts, counts, targets, arguments_node, at),
                            self::callable_element(starts, counts, targets, params, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value183,
                            ),
                            source_ml_source2_concat_value184,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (plan, at) = (plan_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_parameter_ownership_loop44(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut owner: f64,
    mut parameters: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (parameters >= 0f64) && (at < (self::read_required_number_fact(counts, parameters))) {
            {
                let plan_2 = {
                    let source_ml_source2_concat_value185 = plan;
                    {
                        let source_ml_source2_concat_value186 = vec![
                            owner,
                            at,
                            self::callable_element(starts, counts, targets, parameters, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value185,
                            ),
                            source_ml_source2_concat_value186,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (plan, at) = (plan_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_function_children_loop45(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut params: f64,
    mut children: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (params >= 0f64) && (at < (self::read_required_number_fact(counts, params))) {
            {
                let children_2 = {
                    let source_ml_source2_concat_value187 = children;
                    {
                        let source_ml_source2_concat_value188 =
                            vec![self::callable_element(starts, counts, targets, params, at)];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value187,
                            ),
                            source_ml_source2_concat_value188,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (children, at) = (children_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { children };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result {
    MlSource1TemporaryReadCallableClosureParameterErrorLoop47Done,
    MlSource1TemporaryReadCallableClosureParameterErrorLoop47Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_closure_parameter_error_loop47(
    defaults: &[bool],
    mut at: f64,
) -> self::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result {
    loop {
        return if at < (defaults.len() as f64) {
            if self::read_required_boolean_fact(defaults, at) {
                self::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Return(String::from("closure parameter default"))
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
            self::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Result::MlSource1TemporaryReadCallableClosureParameterErrorLoop47Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadCallableClosureParameterErrorLoop46Result {
    MlSource1TemporaryReadCallableClosureParameterErrorLoop46Done(f64),
    MlSource1TemporaryReadCallableClosureParameterErrorLoop46Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadCallableClosureParameterErrorLoop48Result {
    MlSource1TemporaryReadCallableClosureParameterErrorLoop48Done,
    MlSource1TemporaryReadCallableClosureParameterErrorLoop48Return(String),
}
