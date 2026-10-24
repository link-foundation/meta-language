// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableTypeParametersJoin6;
use super::MlSource1TemporaryCallableTypeParametersLoop5Result;
use super::callable_assigned_expression_references;
use super::callable_element;
use super::callable_field;
use super::callable_tag;
use super::callable_text;
use super::callable_truthy;
use super::callable_type_parameter_segments;
use super::callable_type_parameters;
use super::callable_type_trim_end;
use super::callable_type_trim_start;
use super::ml_array;
use super::read_callable_free_references;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_type_parameters_loop5(
    units: &[f64],
    mut start: f64,
    mut named: bool,
    part: &[f64],
    commas: &[f64],
    mut from: f64,
    mut at: f64,
    mut count: f64,
    mut plan: Vec<f64>,
) -> self::MlSource1TemporaryCallableTypeParametersLoop5Result {
    loop {
        return if at <= (commas.len() as f64) {
            {
                let to = if at == (commas.len() as f64) {
                    part.len() as f64
                } else {
                    self::read_required_number_fact(commas, at)
                };
                {
                    let begin = self::callable_type_trim_start(part, from, to);
                    {
                        let finish = self::callable_type_trim_end(part, begin, to);
                        {
                            let source_ml_source2_s100 = if finish > begin {
                                {
                                    let count_2 = count + 1f64;
                                    {
                                        let plan_2 = {
                                            let source_ml_source2_concat_value101 = plan;
                                            {
                                                let source_ml_source2_concat_value102 =
                                                    self::callable_type_parameter_segments(
                                                        units,
                                                        (start + begin),
                                                        (start + finish),
                                                        named,
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value101,
                                                    ),
                                                    source_ml_source2_concat_value102,
                                                )
                                            }
                                        };
                                        self::MlSource1TemporaryCallableTypeParametersJoin6::MlSource1TemporaryCallableTypeParametersJoin6Next(count_2, plan_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryCallableTypeParametersJoin6::MlSource1TemporaryCallableTypeParametersJoin6Next(count, plan)
                            };
                            match source_ml_source2_s100 {
                                self::MlSource1TemporaryCallableTypeParametersJoin6::MlSource1TemporaryCallableTypeParametersJoin6Next(count_3, plan_3) => {
                                    {
                                        let from_2 = to + 1f64;
                                        {
                                            let at_2 = at + 1f64;
                                            {
                                                (from, at, count, plan) = (from_2, at_2, count_3, plan_3);
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
        } else {
            self::MlSource1TemporaryCallableTypeParametersLoop5Result::MlSource1TemporaryCallableTypeParametersLoop5Done(count, plan)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_type_syntax_loop8(
    units: &[f64],
    prefix: &[f64],
    mut at: f64,
) -> f64 {
    loop {
        if ((at < (prefix.len() as f64)) && (at < (units.len() as f64)))
            && ((self::read_required_number_fact(units, at))
                == (self::read_required_number_fact(prefix, at)))
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableTypeSyntaxJoin9 {
    MlSource1TemporaryPlanCallableTypeSyntaxJoin9Next,
    MlSource1TemporaryPlanCallableTypeSyntaxJoin9Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result {
    MlSource1TemporaryPlanCallableTypeSyntaxLoop10Done,
    MlSource1TemporaryPlanCallableTypeSyntaxLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_type_syntax_loop10(
    units: &[f64],
    mut open: f64,
    mut close: f64,
) -> self::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result {
    loop {
        return if close > (open + 1f64) {
            {
                let close_2 = close - 1f64;
                {
                    let colon = self::callable_type_trim_start(
                        units,
                        (close_2 + 1f64),
                        (units.len() as f64),
                    );
                    if ((((self::read_required_number_fact(units, close_2)) == 41f64)
                        && (colon < (units.len() as f64)))
                        && ((self::read_required_number_fact(units, colon)) == 58f64))
                        && ((colon + 1f64) < (units.len() as f64))
                    {
                        {
                            let result = self::callable_type_trim_start(
                                units,
                                (colon + 1f64),
                                (units.len() as f64),
                            );
                            self::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Return({
                                let source_ml_source2_concat_value103 = vec![1f64, if result < (units.len() as f64) {
                                    result
                                } else {
                                    (units.len() as f64) - 1f64
                                }, (units.len() as f64)];
                                {
                                    let source_ml_source2_concat_value104 = self::callable_type_parameters(units, (open + 1f64), close_2, false);
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value103), source_ml_source2_concat_value104)
                                }
                            })
                        }
                    } else {
                        {
                            close = close_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Result::MlSource1TemporaryPlanCallableTypeSyntaxLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableTypeSyntaxJoin7 {
    MlSource1TemporaryPlanCallableTypeSyntaxJoin7Next,
    MlSource1TemporaryPlanCallableTypeSyntaxJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanCallableTypeSyntaxJoin12 {
    MlSource1TemporaryPlanCallableTypeSyntaxJoin12Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_callable_type_syntax_loop11(
    units: &[f64],
    mut depth: f64,
    mut close: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if (at < (units.len() as f64)) && (close < 0f64) {
            {
                let unit = self::read_required_number_fact(units, at);
                {
                    let depth_3 = if unit == 40f64 {
                        { depth + 1f64 }
                    } else {
                        depth
                    };
                    {
                        let source_ml_source2_s105 = if unit == 41f64 {
                            {
                                let depth_4 = depth_3 - 1f64;
                                if depth_4 == 0f64 {
                                    {
                                        let close_2 = at;
                                        self::MlSource1TemporaryPlanCallableTypeSyntaxJoin12::MlSource1TemporaryPlanCallableTypeSyntaxJoin12Next(depth_4, close_2)
                                    }
                                } else {
                                    self::MlSource1TemporaryPlanCallableTypeSyntaxJoin12::MlSource1TemporaryPlanCallableTypeSyntaxJoin12Next(depth_4, close)
                                }
                            }
                        } else {
                            self::MlSource1TemporaryPlanCallableTypeSyntaxJoin12::MlSource1TemporaryPlanCallableTypeSyntaxJoin12Next(depth_3, close)
                        };
                        match source_ml_source2_s105 {
                            self::MlSource1TemporaryPlanCallableTypeSyntaxJoin12::MlSource1TemporaryPlanCallableTypeSyntaxJoin12Next(depth_5, close_3) => {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (depth, close, at) = (depth_5, close_3, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            close
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableFieldLoop13Result {
    MlSource1TemporaryCallableFieldLoop13Done,
    MlSource1TemporaryCallableFieldLoop13Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_field_loop13(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    field: &str,
    mut at: f64,
) -> self::MlSource1TemporaryCallableFieldLoop13Result {
    loop {
        return if at
            < (self::read_required_number_fact(starts, node)
                + self::read_required_number_fact(counts, node))
        {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlSource1TemporaryCallableFieldLoop13Result::MlSource1TemporaryCallableFieldLoop13Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryCallableFieldLoop13Result::MlSource1TemporaryCallableFieldLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCallableBoundLoop14Result {
    MlSource1TemporaryCallableBoundLoop14Done,
    MlSource1TemporaryCallableBoundLoop14Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_bound_loop14(
    names: &[String],
    name: &str,
    mut at: f64,
) -> self::MlSource1TemporaryCallableBoundLoop14Result {
    loop {
        return if at > 0f64 {
            {
                let at_2 = at - 1f64;
                if (self::read_required_text_fact(names, at_2)) == name {
                    self::MlSource1TemporaryCallableBoundLoop14Result::MlSource1TemporaryCallableBoundLoop14Return(true)
                } else {
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCallableBoundLoop14Result::MlSource1TemporaryCallableBoundLoop14Done
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
pub fn source_ml_source1_temporary_callable_parameter_names_loop15(
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
                    let source_ml_source2_concat_value106 = names;
                    {
                        let source_ml_source2_concat_value107 = vec![self::callable_tag(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_element(starts, counts, targets, parameters, at),
                            "name",
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<String>::new(),
                                source_ml_source2_concat_value106,
                            ),
                            source_ml_source2_concat_value107,
                        )
                    }
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_callable_free_references_loop16(
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
                        let source_ml_source2_concat_value108 = references;
                        {
                            let source_ml_source2_concat_value109 =
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
                                    source_ml_source2_concat_value108,
                                ),
                                source_ml_source2_concat_value109,
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
pub fn source_ml_source1_temporary_read_callable_free_references_loop18(
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
                                let source_ml_source2_concat_value110 = names;
                                {
                                    let source_ml_source2_concat_value111 =
                                        vec![self::callable_text(texts, binding)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source2_concat_value110,
                                        ),
                                        source_ml_source2_concat_value111,
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
pub fn source_ml_source1_temporary_read_callable_free_references_loop17(
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
                                                self::source_ml_source1_temporary_read_callable_free_references_loop18(forms, texts, starts, counts, targets, names, binds, index)
                                            }
                                        }
                                    }
                                };
                                {
                                    let references_2 = {
                                        let source_ml_source2_concat_value114 = references;
                                        {
                                            let source_ml_source2_concat_value115 =
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
                                                        let source_ml_source2_concat_value112 =
                                                            bound.to_vec();
                                                        {
                                                            let source_ml_source2_concat_value113 =
                                                                names_4;
                                                            self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source2_concat_value112), source_ml_source2_concat_value113)
                                                        }
                                                    }),
                                                    checked,
                                                );
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value114,
                                                ),
                                                source_ml_source2_concat_value115,
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
pub fn source_ml_source1_temporary_read_callable_free_references_loop19(
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
                                let source_ml_source2_concat_value116 = references;
                                {
                                    let source_ml_source2_concat_value117 =
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
                                            source_ml_source2_concat_value116,
                                        ),
                                        source_ml_source2_concat_value117,
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
pub fn source_ml_source1_temporary_callable_read_events_loop20(
    references: &[f64],
    mut events: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (references.len() as f64) {
            {
                let events_2 = {
                    let source_ml_source2_concat_value118 = events;
                    {
                        let source_ml_source2_concat_value119 =
                            vec![0f64, self::read_required_number_fact(references, at)];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value118,
                            ),
                            source_ml_source2_concat_value119,
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
pub fn source_ml_source1_temporary_callable_assigned_expression_references_loop21(
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
                        let source_ml_source2_concat_value120 = assigned;
                        {
                            let source_ml_source2_concat_value121 =
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
                                    source_ml_source2_concat_value120,
                                ),
                                source_ml_source2_concat_value121,
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
pub fn source_ml_source1_temporary_callable_assigned_expression_references_loop22(
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
                                    let source_ml_source2_concat_value122 = assigned;
                                    {
                                        let source_ml_source2_concat_value123 =
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
                                                source_ml_source2_concat_value122,
                                            ),
                                            source_ml_source2_concat_value123,
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
pub fn source_ml_source1_temporary_read_callable_statement_uses_loop24(
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
                            let source_ml_source2_concat_value124 = scope;
                            {
                                let source_ml_source2_concat_value125 = vec![self::callable_text(
                                    texts,
                                    self::read_required_number_fact(next, (index + 1f64)),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source2_concat_value124,
                                    ),
                                    source_ml_source2_concat_value125,
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
