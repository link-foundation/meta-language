// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::callable_bound;
use super::callable_element;
use super::callable_field;
use super::callable_nested_events;
use super::callable_outer_assignments;
use super::callable_statement_uses;
use super::callable_tag;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::ml_read_callable_free_references_loop18;
use super::read_callable_free_references;
use super::read_callable_statement_uses;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_free_references_loop17(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bound: &[String],
    mut checked: bool,
    mut references: Vec<f64>,
    mut cases: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (cases >= 0f64) && (at < (self::read_required_number_fact(counts, cases))) {
            {
                let branch = self::callable_element(starts, counts, targets, cases, at);
                {
                    let pattern =
                        self::callable_field(starts, counts, labels, targets, branch, "pattern");
                    {
                        let pattern_kind = self::callable_tag(
                            texts, starts, counts, labels, targets, pattern, "k",
                        );
                        {
                            let names = Vec::<String>::new();
                            {
                                let names_4 = if (pattern_kind == "bind")
                                    || (pattern_kind == "natSucc")
                                {
                                    {
                                        vec![self::callable_tag(
                                            texts, starts, counts, labels, targets, pattern, "name",
                                        )]
                                    }
                                } else {
                                    {
                                        let binds = self::callable_field(
                                            starts, counts, labels, targets, pattern, "binds",
                                        );
                                        {
                                            let index = 0f64;
                                            {
                                                self::ml_read_callable_free_references_loop18(
                                                    forms, texts, starts, counts, targets, names,
                                                    binds, index,
                                                )
                                            }
                                        }
                                    }
                                };
                                {
                                    let references_2 = {
                                        let (ml_append_right, ml_append_left) = (
                                            self::read_callable_free_references(
                                                forms,
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                self::callable_field(
                                                    starts, counts, labels, targets, branch, "body",
                                                ),
                                                &(self::ml_array::append(bound.to_vec(), names_4)),
                                                checked,
                                            ),
                                            references,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (references, at) = (references_2, at_2);
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
        return { references };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_free_references_loop19(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    bound: &[String],
    mut checked: bool,
    mut references: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            {
                let label = self::read_required_text_fact(labels, at);
                {
                    let references_3 = if ((label != "span") && (label != "type"))
                        && (checked || (label != "data"))
                    {
                        {
                            {
                                let (ml_append_right, ml_append_left) = (
                                    self::read_callable_free_references(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        self::read_required_number_fact(targets, at),
                                        bound,
                                        checked,
                                    ),
                                    references,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        references
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (references, at) = (references_3, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { references };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_read_events_loop20(
    references: &[f64],
    mut events: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (references.len() as f64) {
            {
                let events_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![0f64, self::read_required_number_fact(references, at)],
                        events,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
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
        return { events };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_callable_statement_uses_loop22(
    texts: &[String],
    mut scope: Vec<String>,
    next: &[f64],
    mut index: f64,
) -> Vec<String> {
    loop {
        if index < (next.len() as f64) {
            {
                let scope_3 = if (self::read_required_number_fact(next, index)) == 2f64 {
                    {
                        {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(next, (index + 1f64)),
                                )],
                                scope,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        }
                    }
                } else {
                    scope
                };
                {
                    let index_2 = index + 2f64;
                    {
                        (scope, index) = (scope_3, index_2);
                        continue;
                    }
                }
            }
        }
        return { scope };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_statement_uses_loop21(
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
                            let scope_2 = self::ml_read_callable_statement_uses_loop22(
                                texts,
                                scope,
                                &(next),
                                index,
                            );
                            {
                                let events_2 = {
                                    let (ml_append_right, ml_append_left) = (next, events);
                                    self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_nested_events_loop23(
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
                            let (ml_append_right, ml_append_left) = (
                                vec![
                                    self::read_required_number_fact(events, at),
                                    self::read_required_number_fact(events, (at + 1f64)),
                                ],
                                result,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_statement_uses_loop24(
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
                        let (ml_append_right, ml_append_left) = (
                            self::callable_nested_events(
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
                            ),
                            events,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_statement_uses_loop26(
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
                            let (ml_append_right, ml_append_left) = (
                                vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(update, (at + 1f64)),
                                )],
                                inner,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
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
pub enum MlCallableStatementUsesLoop25Result {
    MlCallableStatementUsesLoop25Done(Vec<String>, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_statement_uses_loop25(
    texts: &[String],
    initial: &[f64],
    mut inner: Vec<String>,
    mut at: f64,
) -> self::MlCallableStatementUsesLoop25Result {
    loop {
        if at < (initial.len() as f64) {
            {
                let inner_3 = if (self::read_required_number_fact(initial, at)) == 2f64 {
                    {
                        {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(initial, (at + 1f64)),
                                )],
                                inner,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
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
            self::MlCallableStatementUsesLoop25Result::MlCallableStatementUsesLoop25Done(inner, at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadCallableCaptureConflictsJoin30 {
    MlReadCallableCaptureConflictsJoin30Next(Vec<f64>, bool),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_capture_conflicts_loop29(
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
                    let ml_s25 = if self::callable_bound(
                        assigned,
                        &(self::callable_text(texts, capture)),
                    ) {
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![self::read_required_number_fact(closures, at), capture],
                                    result,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let found_2 = true;
                                self::MlReadCallableCaptureConflictsJoin30::MlReadCallableCaptureConflictsJoin30Next(result_2, found_2)
                            }
                        }
                    } else {
                        self::MlReadCallableCaptureConflictsJoin30::MlReadCallableCaptureConflictsJoin30Next(result, found)
                    };
                    match ml_s25 {
                        self::MlReadCallableCaptureConflictsJoin30::MlReadCallableCaptureConflictsJoin30Next(result_3, found_3) => {
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

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_capture_conflicts_loop28(
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
                            let result_2 = self::ml_read_callable_capture_conflicts_loop29(
                                texts, starts, counts, targets, assigned, closures, at, result,
                                captures, index, found,
                            );
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
pub enum MlReadCallableCaptureConflictsLoop27Result {
    MlReadCallableCaptureConflictsLoop27Done(Vec<String>, Vec<f64>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadCallableCaptureConflictsJoin31 {
    MlReadCallableCaptureConflictsJoin31Next(Vec<String>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_read_callable_capture_conflicts_loop27(
    texts: &[String],
    observations: &[f64],
    mut assigned: Vec<String>,
    mut closures: Vec<f64>,
    mut at: f64,
) -> self::MlReadCallableCaptureConflictsLoop27Result {
    loop {
        return if at < (observations.len() as f64) {
            {
                let ml_s26 = if (self::read_required_number_fact(observations, at)) == 0f64 {
                    {
                        let assigned_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(observations, (at + 1f64)),
                                )],
                                assigned,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        self::MlReadCallableCaptureConflictsJoin31::MlReadCallableCaptureConflictsJoin31Next(assigned_2, closures)
                    }
                } else {
                    {
                        let closures_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::read_required_number_fact(observations, (at + 1f64))],
                                closures,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        self::MlReadCallableCaptureConflictsJoin31::MlReadCallableCaptureConflictsJoin31Next(assigned, closures_2)
                    }
                };
                match ml_s26 {
                    self::MlReadCallableCaptureConflictsJoin31::MlReadCallableCaptureConflictsJoin31Next(assigned_3, closures_3) => {
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
            self::MlReadCallableCaptureConflictsLoop27Result::MlReadCallableCaptureConflictsLoop27Done(assigned, closures, at)
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_outer_assignments_loop32(
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
                            let (ml_append_right, ml_append_left) = (
                                self::callable_outer_assignments(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::read_required_number_fact(targets, at),
                                ),
                                result,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_parameter_types_loop33(
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
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        parameter,
                                        type_,
                                        if self::callable_truthy(forms, texts, explicit) {
                                            1f64
                                        } else {
                                            0f64
                                        },
                                    ],
                                    result,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_function_type_loop34(
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
                        let (ml_append_right, ml_append_left) = (
                            vec![self::callable_field(
                                starts, counts, labels, targets, parameter, "type",
                            )],
                            plan,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_type_constraints_loop35(
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
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::callable_element(starts, counts, targets, left_params, at),
                            self::callable_element(starts, counts, targets, right_params, at),
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
