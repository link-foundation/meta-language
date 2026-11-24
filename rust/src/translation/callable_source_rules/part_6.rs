// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::callable_assigned_expression_references;
use super::callable_element;
use super::callable_field;
use super::callable_tag;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::read_callable_free_references;
use super::read_callable_source_pattern_names;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop17(
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
                        let source_ml_source2_concat_value113 = references;
                        {
                            let source_ml_source2_concat_value114 =
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
                                );
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value113,
                                ),
                                source_ml_source2_concat_value114,
                            )
                        }
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop19(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut patterns: f64,
    mut index: f64,
    mut names: Vec<String>,
) -> Vec<String> {
    loop {
        if (patterns >= 0f64) && (index < (self::read_required_number_fact(counts, patterns))) {
            {
                let names_2 = {
                    let source_ml_source2_concat_value115 = names;
                    {
                        let source_ml_source2_concat_value116 =
                            self::read_callable_source_pattern_names(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_element(starts, counts, targets, patterns, index),
                            );
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<String>::new(),
                                source_ml_source2_concat_value115,
                            ),
                            source_ml_source2_concat_value116,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (index, names) = (index_2, names_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop18(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bound: &[String],
    mut checked: bool,
    mut references: Vec<f64>,
    mut rows: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (rows >= 0f64) && (at < (self::read_required_number_fact(counts, rows))) {
            {
                let row = self::callable_element(starts, counts, targets, rows, at);
                {
                    let patterns =
                        self::callable_field(starts, counts, labels, targets, row, "patterns");
                    {
                        let index = 0f64;
                        {
                            let names = Vec::<String>::new();
                            {
                                let names_2 = self::source_ml_source1_temporary_read_callable_free_references_loop19(texts, starts, counts, labels, targets, patterns, index, names);
                                {
                                    let references_2 = {
                                        let source_ml_source2_concat_value119 = references;
                                        {
                                            let source_ml_source2_concat_value120 =
                                                self::read_callable_free_references(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    self::callable_field(
                                                        starts, counts, labels, targets, row,
                                                        "guard",
                                                    ),
                                                    &({
                                                        let source_ml_source2_concat_value117 =
                                                            bound.to_vec();
                                                        {
                                                            let source_ml_source2_concat_value118 =
                                                                names_2.clone();
                                                            self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value117), source_ml_source2_concat_value118)
                                                        }
                                                    }),
                                                    checked,
                                                );
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value119,
                                                ),
                                                source_ml_source2_concat_value120,
                                            )
                                        }
                                    };
                                    {
                                        let references_3 = {
                                            let source_ml_source2_concat_value123 = references_2;
                                            {
                                                let source_ml_source2_concat_value124 =
                                                    self::read_callable_free_references(
                                                        forms,
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        self::callable_field(
                                                            starts, counts, labels, targets, row,
                                                            "body",
                                                        ),
                                                        &({
                                                            let source_ml_source2_concat_value121 =
                                                                bound.to_vec();
                                                            {
                                                                let source_ml_source2_concat_value122 =
                                                                    names_2;
                                                                self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value121), source_ml_source2_concat_value122)
                                                            }
                                                        }),
                                                        checked,
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value123,
                                                    ),
                                                    source_ml_source2_concat_value124,
                                                )
                                            }
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
                        }
                    }
                }
            }
        }
        return { references };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop21(
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
                                let source_ml_source2_concat_value125 = names;
                                {
                                    let source_ml_source2_concat_value126 =
                                        vec![self::callable_text(texts, binding)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source2_concat_value125,
                                        ),
                                        source_ml_source2_concat_value126,
                                    )
                                }
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop20(
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
                                                self::source_ml_source1_temporary_read_callable_free_references_loop21(forms, texts, starts, counts, targets, names, binds, index)
                                            }
                                        }
                                    }
                                };
                                {
                                    let references_2 = {
                                        let source_ml_source2_concat_value129 = references;
                                        {
                                            let source_ml_source2_concat_value130 =
                                                self::read_callable_free_references(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    self::callable_field(
                                                        starts, counts, labels, targets, branch,
                                                        "body",
                                                    ),
                                                    &({
                                                        let source_ml_source2_concat_value127 =
                                                            bound.to_vec();
                                                        {
                                                            let source_ml_source2_concat_value128 =
                                                                names_4;
                                                            self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value127), source_ml_source2_concat_value128)
                                                        }
                                                    }),
                                                    checked,
                                                );
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value129,
                                                ),
                                                source_ml_source2_concat_value130,
                                            )
                                        }
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop22(
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
                                let source_ml_source2_concat_value131 = references;
                                {
                                    let source_ml_source2_concat_value132 =
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
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value131,
                                        ),
                                        source_ml_source2_concat_value132,
                                    )
                                }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_read_events_loop23(
    references: &[f64],
    mut events: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (references.len() as f64) {
            {
                let events_2 = {
                    let source_ml_source2_concat_value133 = events;
                    {
                        let source_ml_source2_concat_value134 =
                            vec![0f64, self::read_required_number_fact(references, at)];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value133,
                            ),
                            source_ml_source2_concat_value134,
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
        return { events };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_assigned_expression_references_loop25(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut patterns: f64,
    mut index: f64,
    mut names: Vec<String>,
) -> Vec<String> {
    loop {
        if (patterns >= 0f64) && (index < (self::read_required_number_fact(counts, patterns))) {
            {
                let names_2 = {
                    let source_ml_source2_concat_value135 = names;
                    {
                        let source_ml_source2_concat_value136 =
                            self::read_callable_source_pattern_names(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_element(starts, counts, targets, patterns, index),
                            );
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<String>::new(),
                                source_ml_source2_concat_value135,
                            ),
                            source_ml_source2_concat_value136,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (index, names) = (index_2, names_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_assigned_expression_references_loop24(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bound: &[String],
    mut references: Vec<f64>,
    mut rows: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (rows >= 0f64) && (at < (self::read_required_number_fact(counts, rows))) {
            {
                let row = self::callable_element(starts, counts, targets, rows, at);
                {
                    let patterns =
                        self::callable_field(starts, counts, labels, targets, row, "patterns");
                    {
                        let index = 0f64;
                        {
                            let names = Vec::<String>::new();
                            {
                                let names_2 = self::source_ml_source1_temporary_callable_assigned_expression_references_loop25(texts, starts, counts, labels, targets, patterns, index, names);
                                {
                                    let references_2 = {
                                        let source_ml_source2_concat_value139 = references;
                                        {
                                            let source_ml_source2_concat_value140 =
                                                self::callable_assigned_expression_references(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    self::callable_field(
                                                        starts, counts, labels, targets, row,
                                                        "guard",
                                                    ),
                                                    &({
                                                        let source_ml_source2_concat_value137 =
                                                            bound.to_vec();
                                                        {
                                                            let source_ml_source2_concat_value138 =
                                                                names_2.clone();
                                                            self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value137), source_ml_source2_concat_value138)
                                                        }
                                                    }),
                                                );
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value139,
                                                ),
                                                source_ml_source2_concat_value140,
                                            )
                                        }
                                    };
                                    {
                                        let references_3 = {
                                            let source_ml_source2_concat_value143 = references_2;
                                            {
                                                let source_ml_source2_concat_value144 =
                                                    self::callable_assigned_expression_references(
                                                        forms,
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        self::callable_field(
                                                            starts, counts, labels, targets, row,
                                                            "body",
                                                        ),
                                                        &({
                                                            let source_ml_source2_concat_value141 =
                                                                bound.to_vec();
                                                            {
                                                                let source_ml_source2_concat_value142 =
                                                                    names_2;
                                                                self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value141), source_ml_source2_concat_value142)
                                                            }
                                                        }),
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value143,
                                                    ),
                                                    source_ml_source2_concat_value144,
                                                )
                                            }
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
                        }
                    }
                }
            }
        }
        return { references };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_assigned_expression_references_loop26(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bound: &[String],
    mut params: f64,
    mut assigned: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if (params >= 0f64) && (index < (self::read_required_number_fact(counts, params))) {
            {
                let parameter = self::callable_element(starts, counts, targets, params, index);
                {
                    let assigned_2 = {
                        let source_ml_source2_concat_value145 = assigned;
                        {
                            let source_ml_source2_concat_value146 =
                                self::callable_assigned_expression_references(
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
                                );
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value145,
                                ),
                                source_ml_source2_concat_value146,
                            )
                        }
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (assigned, index) = (assigned_2, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { assigned };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_assigned_expression_references_loop27(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    bound: &[String],
    mut assigned: Vec<f64>,
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
                    let assigned_3 =
                        if ((label != "span") && (label != "type")) && (label != "data") {
                            {
                                {
                                    let source_ml_source2_concat_value147 = assigned;
                                    {
                                        let source_ml_source2_concat_value148 =
                                            self::callable_assigned_expression_references(
                                                forms,
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                self::read_required_number_fact(targets, at),
                                                bound,
                                            );
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value147,
                                            ),
                                            source_ml_source2_concat_value148,
                                        )
                                    }
                                }
                            }
                        } else {
                            assigned
                        };
                    {
                        let at_2 = at + 1f64;
                        {
                            (assigned, at) = (assigned_3, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { assigned };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_callable_statement_uses_loop29(
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
                            let source_ml_source2_concat_value149 = scope;
                            {
                                let source_ml_source2_concat_value150 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(next, (index + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value149,
                                    ),
                                    source_ml_source2_concat_value150,
                                )
                            }
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
