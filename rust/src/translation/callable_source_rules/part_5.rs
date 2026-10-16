// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::*;

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_field_loop13(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    field: &str,
    mut at: f64,
) -> self::MlCallableFieldLoop13Result {
    loop {
        return if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlCallableFieldLoop13Result::MlCallableFieldLoop13Return(
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
            self::MlCallableFieldLoop13Result::MlCallableFieldLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlCallableBoundLoop14Result {
    MlCallableBoundLoop14Done,
    MlCallableBoundLoop14Return(bool),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_bound_loop14(
    names: &[String],
    name: &str,
    mut at: f64,
) -> self::MlCallableBoundLoop14Result {
    loop {
        return if at > 0f64 {
            {
                let at_2 = at - 1f64;
                if (self::read_required_text_fact(names, at_2)) == name {
                    self::MlCallableBoundLoop14Result::MlCallableBoundLoop14Return(true)
                } else {
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlCallableBoundLoop14Result::MlCallableBoundLoop14Done
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
pub fn ml_callable_parameter_names_loop15(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut parameters: f64,
    mut names: Vec<String>,
    mut at: f64,
) -> Vec<String> {
    loop {
        if at < (self::read_required_number_fact(counts, parameters)) {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::callable_tag(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_element(starts, counts, targets, parameters, at),
                            "name",
                        )],
                        names,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (names, at) = (names_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_free_references_loop16(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bound: &[String],
    mut checked: bool,
    mut params: f64,
    mut references: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (!checked && (params >= 0f64))
            && (at < (self::read_required_number_fact(counts, params)))
        {
            {
                let parameter = self::callable_element(starts, counts, targets, params, at);
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
                                    starts, counts, labels, targets, parameter, "default",
                                ),
                                bound,
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
pub fn ml_read_callable_free_references_loop18(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut names: Vec<String>,
    mut binds: f64,
    mut index: f64,
) -> Vec<String> {
    loop {
        if (binds >= 0f64) && (index < (self::read_required_number_fact(counts, binds))) {
            {
                let binding = self::callable_element(starts, counts, targets, binds, index);
                {
                    let names_3 = if self::callable_truthy(forms, texts, binding) {
                        {
                            {
                                let (ml_append_right, ml_append_left) =
                                    (vec![self::callable_text(texts, binding)], names);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        names
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (names, index) = (names_3, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}

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
                let result_3 = if (self::read_required_number_fact(events, at)) != 2f64 {
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
                } else {
                    result
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
