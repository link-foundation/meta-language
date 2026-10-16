// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::*;

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_callable_free_references(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    bound: &[String],
    checked: bool,
) -> Vec<f64> {
    if (node < 0f64)
        || (((self::read_required_text_fact(forms, node)) != "object")
            && ((self::read_required_text_fact(forms, node)) != "array"))
    {
        Vec::<f64>::new()
    } else {
        {
            let kind = self::callable_tag(texts, starts, counts, labels, targets, node, "k");
            if kind == "lambda" {
                {
                    let params =
                        self::callable_field(starts, counts, labels, targets, node, "params");
                    {
                        let names = self::callable_parameter_names(
                            texts, starts, counts, labels, targets, params,
                        );
                        {
                            let references = self::read_callable_free_references(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_field(starts, counts, labels, targets, node, "body"),
                                &(self::ml_array::append(bound.to_vec(), names)),
                                checked,
                            );
                            {
                                let at = 0f64;
                                {
                                    self::ml_read_callable_free_references_loop16(
                                        forms, texts, starts, counts, labels, targets, bound,
                                        checked, params, references, at,
                                    )
                                }
                            }
                        }
                    }
                }
            } else if kind == "let" {
                {
                    let value = self::read_callable_free_references(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, node, "value"),
                        bound,
                        checked,
                    );
                    {
                        let name = self::callable_tag(
                            texts, starts, counts, labels, targets, node, "name",
                        );
                        {
                            let (ml_append_right, ml_append_left) = (
                                self::read_callable_free_references(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::callable_field(
                                        starts, counts, labels, targets, node, "body",
                                    ),
                                    &({
                                        let (ml_append_right, ml_append_left) =
                                            (vec![name], bound.to_vec());
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    }),
                                    checked,
                                ),
                                value,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        }
                    }
                }
            } else if checked && (kind == "match") {
                {
                    let references_3 = self::read_callable_free_references(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, node, "scrutinee"),
                        bound,
                        checked,
                    );
                    {
                        let cases =
                            self::callable_field(starts, counts, labels, targets, node, "cases");
                        {
                            let at_2 = 0f64;
                            {
                                self::ml_read_callable_free_references_loop17(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    bound,
                                    checked,
                                    references_3,
                                    cases,
                                    at_2,
                                )
                            }
                        }
                    }
                }
            } else {
                {
                    let references_5 = Vec::<f64>::new();
                    {
                        let references_7 = if checked && (kind == "var") {
                            {
                                let name_2 = self::callable_tag(
                                    texts, starts, counts, labels, targets, node, "name",
                                );
                                if (self::callable_bound(bound, &(name_2))) {
                                    references_5
                                } else {
                                    { vec![node] }
                                }
                            }
                        } else {
                            references_5
                        };
                        {
                            let references_9 = if checked && (kind == "name") {
                                references_7
                            } else {
                                {
                                    let path = self::callable_field(
                                        starts, counts, labels, targets, node, "path",
                                    );
                                    if (path >= 0f64)
                                        && ((self::read_required_number_fact(counts, path)) == 1f64)
                                    {
                                        {
                                            let name_3 = self::callable_element(
                                                starts, counts, targets, path, 0f64,
                                            );
                                            if (self::callable_bound(
                                                bound,
                                                &(self::callable_text(texts, name_3)),
                                            )) {
                                                references_7
                                            } else {
                                                { vec![name_3] }
                                            }
                                        }
                                    } else {
                                        references_7
                                    }
                                }
                            };
                            {
                                let at_3 = self::read_required_number_fact(starts, node);
                                {
                                    self::ml_read_callable_free_references_loop19(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        bound,
                                        checked,
                                        references_9,
                                        at_3,
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn callable_read_events(references: &[f64]) -> Vec<f64> {
    {
        let events = Vec::<f64>::new();
        {
            let at = 0f64;
            { self::ml_callable_read_events_loop20(references, events, at) }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_callable_statement_uses(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    statements: f64,
    bound: &[String],
) -> Vec<f64> {
    {
        let events = Vec::<f64>::new();
        {
            let scope = bound.to_vec();
            {
                let at = 0f64;
                {
                    self::ml_read_callable_statement_uses_loop21(
                        forms, texts, starts, counts, labels, targets, statements, events, scope,
                        at,
                    )
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn callable_nested_events(events: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            { self::ml_callable_nested_events_loop23(events, result, at) }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn callable_statement_uses(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    statement: f64,
    bound: &[String],
) -> Vec<f64> {
    {
        let role = self::read_callable_statement_role(
            &(self::callable_tag(texts, starts, counts, labels, targets, statement, "s")),
        );
        {
            let name = self::callable_field(starts, counts, labels, targets, statement, "name");
            {
                let value =
                    self::callable_field(starts, counts, labels, targets, statement, "value");
                if role == "declare" {
                    self::ml_array::append(
                        self::callable_read_events(
                            &(self::read_callable_free_references(
                                forms, texts, starts, counts, labels, targets, value, bound, false,
                            )),
                        ),
                        vec![2f64, name],
                    )
                } else if role == "assign" {
                    {
                        let reads = self::callable_read_events(
                            &(self::read_callable_free_references(
                                forms, texts, starts, counts, labels, targets, value, bound, false,
                            )),
                        );
                        if self::callable_bound(bound, &(self::callable_text(texts, name))) {
                            reads
                        } else {
                            {
                                let (ml_append_right, ml_append_left) =
                                    (vec![0f64, name, 1f64, name], reads);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    }
                } else if role == "read" {
                    self::callable_read_events(
                        &(self::read_callable_free_references(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "expr",
                            ),
                            bound,
                            false,
                        )),
                    )
                } else if role == "reference" {
                    {
                        let references = self::read_callable_free_references(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "object",
                            ),
                            bound,
                            false,
                        );
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
                                            starts, counts, labels, targets, statement, "index",
                                        ),
                                        bound,
                                        false,
                                    ),
                                    references,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            self::callable_read_events(
                                &({
                                    let (ml_append_right, ml_append_left) = (
                                        self::read_callable_free_references(
                                            forms, texts, starts, counts, labels, targets, value,
                                            bound, false,
                                        ),
                                        references_2,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }),
                            )
                        }
                    }
                } else if role == "block" {
                    {
                        let events = self::read_callable_statement_uses(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "body",
                            ),
                            bound,
                        );
                        if self::callable_truthy(
                            forms,
                            texts,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "inline",
                            ),
                        ) {
                            events
                        } else {
                            self::callable_nested_events(&(events))
                        }
                    }
                } else if (role == "branch") || (role == "loop") {
                    {
                        let reads_2 = self::callable_read_events(
                            &(self::read_callable_free_references(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_field(
                                    starts, counts, labels, targets, statement, "cond",
                                ),
                                bound,
                                false,
                            )),
                        );
                        if role == "loop" {
                            {
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
                                                starts, counts, labels, targets, statement, "body",
                                            ),
                                            bound,
                                        )),
                                    ),
                                    reads_2,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        } else {
                            {
                                let yes = self::read_callable_statement_uses(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::callable_field(
                                        starts, counts, labels, targets, statement, "then",
                                    ),
                                    bound,
                                );
                                {
                                    let no = self::read_callable_statement_uses(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        self::callable_field(
                                            starts, counts, labels, targets, statement, "else",
                                        ),
                                        bound,
                                    );
                                    self::ml_array::append(
                                        {
                                            let (ml_append_right, ml_append_left) =
                                                (self::callable_nested_events(&(yes)), reads_2);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        },
                                        self::callable_nested_events(&(no)),
                                    )
                                }
                            }
                        }
                    }
                } else if role == "switch" {
                    {
                        let events_2 = self::callable_read_events(
                            &(self::read_callable_free_references(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    statement,
                                    "discriminant",
                                ),
                                bound,
                                false,
                            )),
                        );
                        {
                            let clauses = self::callable_field(
                                starts, counts, labels, targets, statement, "clauses",
                            );
                            {
                                let at = 0f64;
                                {
                                    self::ml_callable_statement_uses_loop24(
                                        forms, texts, starts, counts, labels, targets, bound,
                                        events_2, clauses, at,
                                    )
                                }
                            }
                        }
                    }
                } else if role == "for" {
                    {
                        let initial = self::read_callable_statement_uses(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(
                                starts, counts, labels, targets, statement, "init",
                            ),
                            bound,
                        );
                        {
                            let inner = bound.to_vec();
                            {
                                let at_2 = 0f64;
                                {
                                    let ml_s8 = self::ml_callable_statement_uses_loop25(
                                        texts,
                                        &(initial),
                                        inner,
                                        at_2,
                                    );
                                    match ml_s8 {
                                                                    self::MlCallableStatementUsesLoop25Result::MlCallableStatementUsesLoop25Done(inner_2, at_3) => {
                                                                        {
                                                                            let events_4 = self::callable_nested_events(&(initial));
                                                                            {
                                                                                let events_5 = { let (ml_append_right, ml_append_left) = (self::callable_read_events(&(self::read_callable_free_references(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "cond"), &(inner_2), false))), events_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                {
                                                                                    let update = self::read_callable_statement_uses(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "update"), &(inner_2));
                                                                                    {
                                                                                        let at_4 = 0f64;
                                                                                        {
                                                                                            let inner_3 = self::ml_callable_statement_uses_loop26(texts, inner_2, at_4, &(update));
                                                                                            self::ml_array::append({ let (ml_append_right, ml_append_left) = (self::callable_nested_events(&(update)), events_5); self::ml_array::append(ml_append_left, ml_append_right) }, self::callable_nested_events(&(self::read_callable_statement_uses(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "body"), &(inner_3)))))
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
                    Vec::<f64>::new()
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_capture_conflicts(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<f64> {
    {
        let observations =
            self::callable_outer_assignments(forms, texts, starts, counts, labels, targets, node);
        {
            let assigned = Vec::<String>::new();
            {
                let closures = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        let ml_s9 = self::ml_read_callable_capture_conflicts_loop27(
                            texts,
                            &(observations),
                            assigned,
                            closures,
                            at,
                        );
                        match ml_s9 {
                            self::MlReadCallableCaptureConflictsLoop27Result::MlReadCallableCaptureConflictsLoop27Done(assigned_2, closures_2, at_2) => {
                                {
                                    let result = Vec::<f64>::new();
                                    {
                                        let at_3 = 0f64;
                                        {
                                            self::ml_read_callable_capture_conflicts_loop28(texts, starts, counts, labels, targets, &(assigned_2), &(closures_2), at_3, result)
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn callable_outer_assignments(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<f64> {
    if (node < 0f64)
        || (((self::read_required_text_fact(forms, node)) != "object")
            && ((self::read_required_text_fact(forms, node)) != "array"))
    {
        Vec::<f64>::new()
    } else if (self::callable_tag(texts, starts, counts, labels, targets, node, "k")) == "lambda" {
        vec![1f64, node]
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let result_3 =
                    if (self::callable_tag(texts, starts, counts, labels, targets, node, "s"))
                        == "assign"
                    {
                        {
                            vec![
                                0f64,
                                self::callable_field(starts, counts, labels, targets, node, "name"),
                            ]
                        }
                    } else {
                        result
                    };
                {
                    let at = self::read_required_number_fact(starts, node);
                    {
                        self::ml_callable_outer_assignments_loop32(
                            forms, texts, starts, counts, labels, targets, node, result_3, at,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn accept_callable_function_reference(
    surface_kind: &str,
    expected_kind: &str,
    declaration_kind: &str,
) -> bool {
    (declaration_kind == "fn")
        && ((surface_kind == "functionValue")
            || ((surface_kind == "name") && (expected_kind == "function")))
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn require_callable_computed_application(
    head_kind: &str,
    path_length: f64,
    local_kind: &str,
) -> bool {
    ((head_kind != "name") && (head_kind != "dotCtor"))
        || (((head_kind == "name") && (path_length == 1f64)) && (local_kind == "function"))
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_callable_application_error(
    kind: &str,
    parameters: f64,
    arguments_count: f64,
) -> String {
    if kind != "function" {
        String::from("the called value is not a function")
    } else if parameters != arguments_count {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (String::from("function expects ")),
                (self::ml_number::js_number(parameters))
            )),
            (String::from(" arguments"))
        )
    } else {
        String::from("")
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_parameter_types(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    parameters: f64,
    expected: f64,
) -> Vec<f64> {
    {
        let expected_kind =
            self::callable_tag(texts, starts, counts, labels, targets, expected, "kind");
        {
            let expected_params =
                self::callable_field(starts, counts, labels, targets, expected, "params");
            {
                let result = Vec::<f64>::new();
                {
                    let at = 0f64;
                    {
                        self::ml_plan_callable_parameter_types_loop33(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            parameters,
                            &(expected_kind),
                            expected_params,
                            result,
                            at,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn plan_callable_function_type(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    parameters: f64,
    result: f64,
) -> Vec<f64> {
    {
        let plan = vec![result];
        {
            let at = 0f64;
            {
                self::ml_plan_callable_function_type_loop34(
                    starts, counts, labels, targets, parameters, plan, at,
                )
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn plan_callable_type_constraints(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    left: f64,
    right: f64,
) -> Vec<f64> {
    {
        let left_params = self::callable_field(starts, counts, labels, targets, left, "params");
        {
            let right_params =
                self::callable_field(starts, counts, labels, targets, right, "params");
            if ((left_params < 0f64) || (right_params < 0f64))
                || ((self::read_required_number_fact(counts, left_params))
                    != (self::read_required_number_fact(counts, right_params)))
            {
                vec![-1f64]
            } else {
                {
                    let pairs = Vec::<f64>::new();
                    {
                        let at = 0f64;
                        {
                            let pairs_2 = self::ml_plan_callable_type_constraints_loop35(
                                starts,
                                counts,
                                targets,
                                left_params,
                                right_params,
                                pairs,
                                at,
                            );
                            {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        self::callable_field(
                                            starts, counts, labels, targets, left, "ret",
                                        ),
                                        self::callable_field(
                                            starts, counts, labels, targets, right, "ret",
                                        ),
                                    ],
                                    pairs_2,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    }
                }
            }
        }
    }
}
