// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::callable_bound;
use super::callable_element;
use super::callable_field;
use super::callable_tag;
use super::callable_text;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_assigned_expression_references_loop24;
use super::source_ml_source1_temporary_callable_assigned_expression_references_loop26;
use super::source_ml_source1_temporary_callable_assigned_expression_references_loop27;
use super::source_ml_source1_temporary_callable_nested_events_loop30;
use super::source_ml_source1_temporary_callable_parameter_names_loop15;
use super::source_ml_source1_temporary_callable_read_events_loop23;
use super::source_ml_source1_temporary_read_callable_free_references_loop17;
use super::source_ml_source1_temporary_read_callable_free_references_loop18;
use super::source_ml_source1_temporary_read_callable_free_references_loop20;
use super::source_ml_source1_temporary_read_callable_free_references_loop22;
use super::source_ml_source1_temporary_read_callable_source_pattern_names_loop16;
use super::source_ml_source1_temporary_read_callable_statement_uses_loop28;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_parameter_names(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    parameters: f64,
) -> Vec<String> {
    {
        let names = Vec::<String>::new();
        if parameters < 0f64 {
            names
        } else {
            {
                let at = 0f64;
                {
                    self::source_ml_source1_temporary_callable_parameter_names_loop15(
                        texts, starts, counts, labels, targets, parameters, names, at,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_source_pattern_names(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
) -> Vec<String> {
    if node < 0f64 {
        Vec::<String>::new()
    } else {
        {
            let kind = self::callable_tag(texts, starts, counts, labels, targets, node, "k");
            {
                let names = Vec::<String>::new();
                {
                    let names_3 =
                        if ((kind == "alias") || (kind == "bind")) || (kind == "bindOrCtor") {
                            {
                                {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![self::callable_tag(
                                            texts, starts, counts, labels, targets, node, "name",
                                        )],
                                        names,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        } else {
                            names
                        };
                    if (kind == "alias") || (kind == "natAdd") {
                        {
                            let source_ml_source2_concat_value16 = names_3;
                            {
                                let source_ml_source2_concat_value17 =
                                    self::read_callable_source_pattern_names(
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
                                            node,
                                            &(if kind == "alias" {
                                                String::from("pattern")
                                            } else {
                                                String::from("inner")
                                            }),
                                        ),
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value16,
                                    ),
                                    source_ml_source2_concat_value17,
                                )
                            }
                        }
                    } else {
                        {
                            if (kind == "ctor") || (kind == "tuple") {
                                {
                                    let args = self::callable_field(
                                        starts, counts, labels, targets, node, "args",
                                    );
                                    {
                                        let at = 0f64;
                                        {
                                            self::source_ml_source1_temporary_read_callable_source_pattern_names_loop16(texts, starts, counts, labels, targets, names_3, args, at)
                                        }
                                    }
                                }
                            } else {
                                names_3
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
                                    let source_ml_source2_concat_value18 = bound.to_vec();
                                    {
                                        let source_ml_source2_concat_value19 = names;
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
                            {
                                let at = 0f64;
                                {
                                    self::source_ml_source1_temporary_read_callable_free_references_loop17(forms, texts, starts, counts, labels, targets, bound, checked, params, references, at)
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
                            let source_ml_source2_concat_value22 = value;
                            {
                                let source_ml_source2_concat_value23 =
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
                                            let source_ml_source2_concat_value20 = bound.to_vec();
                                            {
                                                let source_ml_source2_concat_value21 = vec![name];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_source2_concat_value20,
                                                    ),
                                                    source_ml_source2_concat_value21,
                                                )
                                            }
                                        }),
                                        checked,
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value22,
                                    ),
                                    source_ml_source2_concat_value23,
                                )
                            }
                        }
                    }
                }
            } else if !checked && (kind == "match") {
                {
                    let references_3 = self::read_callable_free_references(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, node, "scrutinees"),
                        bound,
                        checked,
                    );
                    {
                        let rows =
                            self::callable_field(starts, counts, labels, targets, node, "rows");
                        {
                            let at_2 = 0f64;
                            {
                                self::source_ml_source1_temporary_read_callable_free_references_loop18(forms, texts, starts, counts, labels, targets, bound, checked, references_3, rows, at_2)
                            }
                        }
                    }
                }
            } else if checked && (kind == "match") {
                {
                    let references_5 = self::read_callable_free_references(
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
                            let at_3 = 0f64;
                            {
                                self::source_ml_source1_temporary_read_callable_free_references_loop20(forms, texts, starts, counts, labels, targets, bound, checked, references_5, cases, at_3)
                            }
                        }
                    }
                }
            } else {
                {
                    let references_7 = Vec::<f64>::new();
                    {
                        let references_9 = if checked && (kind == "var") {
                            {
                                let name_2 = self::callable_tag(
                                    texts, starts, counts, labels, targets, node, "name",
                                );
                                if (self::callable_bound(bound, &(name_2))) {
                                    references_7
                                } else {
                                    { vec![node] }
                                }
                            }
                        } else {
                            references_7
                        };
                        {
                            let references_11 = if !checked && (kind == "name") {
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
                                                references_9
                                            } else {
                                                { vec![name_3] }
                                            }
                                        }
                                    } else {
                                        references_9
                                    }
                                }
                            } else {
                                references_9
                            };
                            {
                                let at_4 = self::read_required_number_fact(starts, node);
                                {
                                    self::source_ml_source1_temporary_read_callable_free_references_loop22(forms, texts, starts, counts, labels, targets, node, bound, checked, references_11, at_4)
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
                self::source_ml_source1_temporary_callable_read_events_loop23(
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
            if kind == "match" {
                {
                    let references = self::callable_assigned_expression_references(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        self::callable_field(starts, counts, labels, targets, node, "scrutinees"),
                        bound,
                    );
                    {
                        let rows =
                            self::callable_field(starts, counts, labels, targets, node, "rows");
                        {
                            let at = 0f64;
                            {
                                self::source_ml_source1_temporary_callable_assigned_expression_references_loop24(forms, texts, starts, counts, labels, targets, bound, references, rows, at)
                            }
                        }
                    }
                }
            } else if kind == "let" {
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
                        let source_ml_source2_concat_value26 = value;
                        {
                            let source_ml_source2_concat_value27 =
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
                                        let source_ml_source2_concat_value24 = bound.to_vec();
                                        {
                                            let source_ml_source2_concat_value25 =
                                                vec![self::callable_tag(
                                                    texts, starts, counts, labels, targets, node,
                                                    "name",
                                                )];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
                                                    source_ml_source2_concat_value24,
                                                ),
                                                source_ml_source2_concat_value25,
                                            )
                                        }
                                    }),
                                );
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value26,
                                ),
                                source_ml_source2_concat_value27,
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
                                let source_ml_source2_concat_value28 = bound.to_vec();
                                {
                                    let source_ml_source2_concat_value29 =
                                        self::callable_parameter_names(
                                            texts, starts, counts, labels, targets, params,
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source2_concat_value28,
                                        ),
                                        source_ml_source2_concat_value29,
                                    )
                                }
                            }),
                        );
                        {
                            let index = 0f64;
                            {
                                self::source_ml_source1_temporary_callable_assigned_expression_references_loop26(forms, texts, starts, counts, labels, targets, bound, params, assigned, index)
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
                            let at_2 = self::read_required_number_fact(starts, node);
                            {
                                self::source_ml_source1_temporary_callable_assigned_expression_references_loop27(forms, texts, starts, counts, labels, targets, node, bound, assigned_5, at_2)
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
        let source_ml_source2_concat_value30 = self::callable_read_events(
            &(self::read_callable_free_references(
                forms, texts, starts, counts, labels, targets, node, bound, false,
            )),
        );
        {
            let source_ml_source2_concat_value31 = self::callable_assigned_expression_references(
                forms, texts, starts, counts, labels, targets, node, bound,
            );
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value30),
                source_ml_source2_concat_value31,
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
                    self::source_ml_source1_temporary_read_callable_statement_uses_loop28(
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
            { self::source_ml_source1_temporary_callable_nested_events_loop30(events, result, at) }
        }
    }
}
