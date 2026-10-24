// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableStatementUsesLoop27Result;
use super::MlSource1TemporaryReadCallableCaptureConflictsLoop29Result;
use super::callable_bound;
use super::callable_element;
use super::callable_field;
use super::callable_outer_assignments;
use super::callable_parameter_names;
use super::callable_tag;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::read_callable_statement_role;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_assigned_expression_references_loop21;
use super::source_ml_source1_temporary_callable_assigned_expression_references_loop22;
use super::source_ml_source1_temporary_callable_nested_events_loop25;
use super::source_ml_source1_temporary_callable_read_events_loop20;
use super::source_ml_source1_temporary_callable_statement_uses_loop26;
use super::source_ml_source1_temporary_callable_statement_uses_loop27;
use super::source_ml_source1_temporary_callable_statement_uses_loop28;
use super::source_ml_source1_temporary_read_callable_capture_conflicts_loop29;
use super::source_ml_source1_temporary_read_callable_capture_conflicts_loop30;
use super::source_ml_source1_temporary_read_callable_free_references_loop16;
use super::source_ml_source1_temporary_read_callable_free_references_loop17;
use super::source_ml_source1_temporary_read_callable_free_references_loop19;
use super::source_ml_source1_temporary_read_callable_statement_uses_loop23;

/// Execute the checked shared source algorithm.
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
                                &({
                                    let source_ml_source2_concat_value16 = bound.to_vec();
                                    {
                                        let source_ml_source2_concat_value17 = names;
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_source2_concat_value16,
                                            ),
                                            source_ml_source2_concat_value17,
                                        )
                                    }
                                }),
                                checked,
                            );
                            {
                                let at = 0f64;
                                {
                                    self::source_ml_source1_temporary_read_callable_free_references_loop16(forms, texts, starts, counts, labels, targets, bound, checked, params, references, at)
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
                            let source_ml_source2_concat_value20 = value;
                            {
                                let source_ml_source2_concat_value21 =
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
                                            let source_ml_source2_concat_value18 = bound.to_vec();
                                            {
                                                let source_ml_source2_concat_value19 = vec![name];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_source2_concat_value18,
                                                    ),
                                                    source_ml_source2_concat_value19,
                                                )
                                            }
                                        }),
                                        checked,
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value20,
                                    ),
                                    source_ml_source2_concat_value21,
                                )
                            }
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
                                self::source_ml_source1_temporary_read_callable_free_references_loop17(forms, texts, starts, counts, labels, targets, bound, checked, references_3, cases, at_2)
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
                            let references_9 = if !checked && (kind == "name") {
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
                            } else {
                                references_7
                            };
                            {
                                let at_3 = self::read_required_number_fact(starts, node);
                                {
                                    self::source_ml_source1_temporary_read_callable_free_references_loop19(forms, texts, starts, counts, labels, targets, node, bound, checked, references_9, at_3)
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
pub fn callable_read_events(references: &[f64]) -> Vec<f64> {
    {
        let events = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::source_ml_source1_temporary_callable_read_events_loop20(
                    references, events, at,
                )
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
pub fn callable_assigned_expression_references(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    bound: &[String],
) -> Vec<f64> {
    if (node < 0f64)
        || (((self::read_required_text_fact(forms, node)) != "object")
            && ((self::read_required_text_fact(forms, node)) != "array"))
    {
        Vec::<f64>::new()
    } else {
        {
            let kind = self::callable_tag(texts, starts, counts, labels, targets, node, "k");
            if kind == "let" {
                {
                    let value = self::callable_assigned_expression_references(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, node, "value"),
                        bound,
                    );
                    {
                        let source_ml_source2_concat_value24 = value;
                        {
                            let source_ml_source2_concat_value25 =
                                self::callable_assigned_expression_references(
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
                                        let source_ml_source2_concat_value22 = bound.to_vec();
                                        {
                                            let source_ml_source2_concat_value23 =
                                                vec![self::callable_tag(
                                                    texts, starts, counts, labels, targets, node,
                                                    "name",
                                                )];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_source2_concat_value22,
                                                ),
                                                source_ml_source2_concat_value23,
                                            )
                                        }
                                    }),
                                );
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value24,
                                ),
                                source_ml_source2_concat_value25,
                            )
                        }
                    }
                }
            } else if kind == "lambda" {
                {
                    let params =
                        self::callable_field(starts, counts, labels, targets, node, "params");
                    {
                        let assigned = self::callable_assigned_expression_references(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_field(starts, counts, labels, targets, node, "body"),
                            &({
                                let source_ml_source2_concat_value26 = bound.to_vec();
                                {
                                    let source_ml_source2_concat_value27 =
                                        self::callable_parameter_names(
                                            texts, starts, counts, labels, targets, params,
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source2_concat_value26,
                                        ),
                                        source_ml_source2_concat_value27,
                                    )
                                }
                            }),
                        );
                        {
                            let index = 0f64;
                            {
                                self::source_ml_source1_temporary_callable_assigned_expression_references_loop21(forms, texts, starts, counts, labels, targets, bound, params, assigned, index)
                            }
                        }
                    }
                }
            } else {
                {
                    let assigned_3 = Vec::<f64>::new();
                    {
                        let assigned_5 = if kind == "cellWrite" {
                            {
                                let cell = self::callable_field(
                                    starts, counts, labels, targets, node, "cell",
                                );
                                {
                                    let path = self::callable_field(
                                        starts, counts, labels, targets, cell, "path",
                                    );
                                    if (((self::callable_tag(
                                        texts, starts, counts, labels, targets, cell, "k",
                                    )) == "name")
                                        && (path >= 0f64))
                                        && ((self::read_required_number_fact(counts, path)) == 1f64)
                                    {
                                        {
                                            let name = self::callable_element(
                                                starts, counts, targets, path, 0f64,
                                            );
                                            if (self::callable_bound(
                                                bound,
                                                &(self::callable_text(texts, name)),
                                            )) {
                                                assigned_3
                                            } else {
                                                { vec![1f64, name] }
                                            }
                                        }
                                    } else {
                                        assigned_3
                                    }
                                }
                            }
                        } else {
                            assigned_3
                        };
                        {
                            let at = self::read_required_number_fact(starts, node);
                            {
                                self::source_ml_source1_temporary_callable_assigned_expression_references_loop22(forms, texts, starts, counts, labels, targets, node, bound, assigned_5, at)
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn callable_expression_events(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    bound: &[String],
) -> Vec<f64> {
    {
        let source_ml_source2_concat_value28 = self::callable_read_events(
            &(self::read_callable_free_references(
                forms, texts, starts, counts, labels, targets, node, bound, false,
            )),
        );
        {
            let source_ml_source2_concat_value29 = self::callable_assigned_expression_references(
                forms, texts, starts, counts, labels, targets, node, bound,
            );
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value28),
                source_ml_source2_concat_value29,
            )
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
                    self::source_ml_source1_temporary_read_callable_statement_uses_loop23(
                        forms, texts, starts, counts, labels, targets, statements, events, scope,
                        at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_nested_events(events: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            { self::source_ml_source1_temporary_callable_nested_events_loop25(events, result, at) }
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
                    {
                        let source_ml_source2_concat_value30 = self::callable_expression_events(
                            forms, texts, starts, counts, labels, targets, value, bound,
                        );
                        {
                            let source_ml_source2_concat_value31 = vec![2f64, name];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value30,
                                ),
                                source_ml_source2_concat_value31,
                            )
                        }
                    }
                } else if role == "assign" {
                    {
                        let reads = self::callable_expression_events(
                            forms, texts, starts, counts, labels, targets, value, bound,
                        );
                        if self::callable_bound(bound, &(self::callable_text(texts, name))) {
                            reads
                        } else {
                            {
                                let source_ml_source2_concat_value32 = reads;
                                {
                                    let source_ml_source2_concat_value33 =
                                        vec![0f64, name, 1f64, name];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value32,
                                        ),
                                        source_ml_source2_concat_value33,
                                    )
                                }
                            }
                        }
                    }
                } else if role == "read" {
                    self::callable_expression_events(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, statement, "expr"),
                        bound,
                    )
                } else if role == "reference" {
                    {
                        let source_ml_source2_concat_value34 = self::callable_expression_events(
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
                        );
                        {
                            let source_ml_source2_concat_value35 = self::callable_expression_events(
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
                            );
                            {
                                let source_ml_source2_concat_value36 =
                                    self::callable_expression_events(
                                        forms, texts, starts, counts, labels, targets, value, bound,
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value34,
                                        ),
                                        source_ml_source2_concat_value35,
                                    ),
                                    source_ml_source2_concat_value36,
                                )
                            }
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
                        let reads_2 = self::callable_expression_events(
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
                        );
                        if role == "loop" {
                            {
                                let source_ml_source2_concat_value37 = reads_2;
                                {
                                    let source_ml_source2_concat_value38 =
                                        self::callable_nested_events(
                                            &(self::read_callable_statement_uses(
                                                forms,
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                self::callable_field(
                                                    starts, counts, labels, targets, statement,
                                                    "body",
                                                ),
                                                bound,
                                            )),
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value37,
                                        ),
                                        source_ml_source2_concat_value38,
                                    )
                                }
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
                                    {
                                        let source_ml_source2_concat_value39 = reads_2;
                                        {
                                            let source_ml_source2_concat_value40 =
                                                self::callable_nested_events(&(yes));
                                            {
                                                let source_ml_source2_concat_value41 =
                                                    self::callable_nested_events(&(no));
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value39,
                                                        ),
                                                        source_ml_source2_concat_value40,
                                                    ),
                                                    source_ml_source2_concat_value41,
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if role == "switch" {
                    {
                        let events_2 = self::callable_expression_events(
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
                        );
                        {
                            let clauses = self::callable_field(
                                starts, counts, labels, targets, statement, "clauses",
                            );
                            {
                                let at = 0f64;
                                {
                                    self::source_ml_source1_temporary_callable_statement_uses_loop26(
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
                                    let source_ml_source2_s42 = self::source_ml_source1_temporary_callable_statement_uses_loop27(texts, &(initial), inner, at_2);
                                    match source_ml_source2_s42 {
                                                                    self::MlSource1TemporaryCallableStatementUsesLoop27Result::MlSource1TemporaryCallableStatementUsesLoop27Done(inner_2, at_3) => {
                                                                        {
                                                                            let events_4 = self::callable_nested_events(&(initial));
                                                                            {
                                                                                let events_5 = {
                                                                                    let source_ml_source2_concat_value43 = events_4;
                                                                                    {
                                                                                        let source_ml_source2_concat_value44 = self::callable_expression_events(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "cond"), &(inner_2));
                                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value43), source_ml_source2_concat_value44)
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let update = self::read_callable_statement_uses(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "update"), &(inner_2));
                                                                                    {
                                                                                        let at_4 = 0f64;
                                                                                        {
                                                                                            let inner_3 = self::source_ml_source1_temporary_callable_statement_uses_loop28(texts, inner_2, at_4, &(update));
                                                                                            {
                                                                                                let source_ml_source2_concat_value45 = events_5;
                                                                                                {
                                                                                                    let source_ml_source2_concat_value46 = self::callable_nested_events(&(update));
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value47 = self::callable_nested_events(&(self::read_callable_statement_uses(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, statement, "body"), &(inner_3))));
                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value45), source_ml_source2_concat_value46), source_ml_source2_concat_value47)
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
                } else {
                    Vec::<f64>::new()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
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
                        let source_ml_source2_s48 = self::source_ml_source1_temporary_read_callable_capture_conflicts_loop29(texts, &(observations), assigned, closures, at);
                        match source_ml_source2_s48 {
                            self::MlSource1TemporaryReadCallableCaptureConflictsLoop29Result::MlSource1TemporaryReadCallableCaptureConflictsLoop29Done(assigned_2, closures_2, at_2) => {
                                {
                                    let result = Vec::<f64>::new();
                                    {
                                        let at_3 = 0f64;
                                        {
                                            self::source_ml_source1_temporary_read_callable_capture_conflicts_loop30(texts, starts, counts, labels, targets, &(assigned_2), &(closures_2), at_3, result)
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
