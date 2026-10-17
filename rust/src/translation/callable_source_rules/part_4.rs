// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::MlCallableRenamedNodeLoop61Result;
use super::MlCallableRenamingBindingLoop59Result;
use super::accept_callable_type_whitespace;
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
use super::ml_callable_renamed_node_loop60;
use super::ml_callable_renamed_node_loop61;
use super::ml_callable_renamed_node_loop62;
use super::ml_callable_renaming_binding_loop59;
use super::ml_plan_callable_source_renaming_loop58;
use super::read_callable_free_references;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_source_renaming(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    scope: f64,
) -> Vec<f64> {
    {
        let bindings = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let bindings_2 = self::ml_plan_callable_source_renaming_loop58(
                    forms, texts, starts, counts, labels, targets, scope, bindings, at,
                );
                self::callable_renamed_node(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(bindings_2),
                    false,
                )
            }
        }
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn callable_renaming_binding(texts: &[String], bindings: &[f64], name: f64) -> Vec<f64> {
    {
        let at = bindings.len() as f64;
        {
            let ml_s21 = self::ml_callable_renaming_binding_loop59(texts, bindings, name, at);
            match ml_s21 {
                self::MlCallableRenamingBindingLoop59Result::MlCallableRenamingBindingLoop59Done => {
                    vec![name, 0f64]
                }
                self::MlCallableRenamingBindingLoop59Result::MlCallableRenamingBindingLoop59Return(ml_result59) => {
                    ml_result59
                }
            }
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
pub fn callable_renamed_node(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    bindings: &[f64],
    carrier: bool,
) -> Vec<f64> {
    if (node < 0f64)
        || (((self::read_required_text_fact(forms, node)) != "object")
            && ((self::read_required_text_fact(forms, node)) != "array"))
    {
        vec![0f64, node]
    } else if (self::read_required_text_fact(forms, node)) == "array" {
        {
            let plan = vec![1f64, self::read_required_number_fact(counts, node)];
            {
                let at = 0f64;
                {
                    self::ml_callable_renamed_node_loop60(
                        forms, texts, starts, counts, labels, targets, node, bindings, plan, at,
                    )
                }
            }
        }
    } else {
        {
            let kind = self::callable_tag(texts, starts, counts, labels, targets, node, "k");
            {
                let path = self::callable_field(starts, counts, labels, targets, node, "path");
                if ((kind == "name") && (path >= 0f64))
                    && ((self::read_required_number_fact(counts, path)) == 1f64)
                {
                    {
                        let binding = self::callable_renaming_binding(
                            texts,
                            bindings,
                            self::callable_element(starts, counts, targets, path, 0f64),
                        );
                        {
                            let renamed = vec![
                                2f64,
                                node,
                                1f64,
                                -1f64,
                                1f64,
                                1f64,
                                0f64,
                                self::read_required_number_fact(&(binding), 0f64),
                            ];
                            if carrier
                                || ((self::read_required_number_fact(&(binding), 1f64)) == 0f64)
                            {
                                renamed
                            } else {
                                {
                                    let span = self::callable_field(
                                        starts, counts, labels, targets, node, "span",
                                    );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            vec![2f64, -1f64, 3f64, -7f64, 3f64, 0f64, -4f64],
                                            renamed,
                                        ),
                                        vec![-8f64, 0f64, span],
                                    )
                                }
                            }
                        }
                    }
                } else if (kind == "cellRead") || (kind == "cellWrite") {
                    {
                        let cell =
                            self::callable_field(starts, counts, labels, targets, node, "cell");
                        {
                            let value = self::callable_field(
                                starts, counts, labels, targets, node, "value",
                            );
                            {
                                let body = self::callable_field(
                                    starts, counts, labels, targets, node, "body",
                                );
                                {
                                    let has_value = self::callable_truthy(forms, texts, value);
                                    {
                                        let has_body = self::callable_truthy(forms, texts, body);
                                        {
                                            let plan_3 = self::ml_array::append(
                                                vec![
                                                    2f64,
                                                    node,
                                                    ((1f64 + if has_value { 1f64 } else { 0f64 })
                                                        + if has_body { 1f64 } else { 0f64 }),
                                                    -4f64,
                                                ],
                                                self::callable_renamed_node(
                                                    forms, texts, starts, counts, labels, targets,
                                                    cell, bindings, true,
                                                ),
                                            );
                                            {
                                                let plan_5 = if has_value {
                                                    {
                                                        self::ml_array::append(
                                                            {
                                                                let (
                                                                    ml_append_right,
                                                                    ml_append_left,
                                                                ) = (vec![-5f64], plan_3);
                                                                self::ml_array::append(
                                                                    ml_append_left,
                                                                    ml_append_right,
                                                                )
                                                            },
                                                            self::callable_renamed_node(
                                                                forms, texts, starts, counts,
                                                                labels, targets, value, bindings,
                                                                false,
                                                            ),
                                                        )
                                                    }
                                                } else {
                                                    plan_3
                                                };
                                                {
                                                    if has_body {
                                                        {
                                                            self::ml_array::append(
                                                                {
                                                                    let (
                                                                        ml_append_right,
                                                                        ml_append_left,
                                                                    ) = (vec![-3f64], plan_5);
                                                                    self::ml_array::append(
                                                                        ml_append_left,
                                                                        ml_append_right,
                                                                    )
                                                                },
                                                                self::callable_renamed_node(
                                                                    forms, texts, starts, counts,
                                                                    labels, targets, body,
                                                                    bindings, false,
                                                                ),
                                                            )
                                                        }
                                                    } else {
                                                        plan_5
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if kind == "lambda" {
                    {
                        let params =
                            self::callable_field(starts, counts, labels, targets, node, "params");
                        {
                            let inner = bindings.to_vec();
                            {
                                let parameters =
                                    vec![1f64, self::read_required_number_fact(counts, params)];
                                {
                                    let at_2 = 0f64;
                                    {
                                        let ml_s22 = self::ml_callable_renamed_node_loop61(
                                            forms, texts, starts, counts, labels, targets,
                                            bindings, params, inner, parameters, at_2,
                                        );
                                        match ml_s22 {
                                                        self::MlCallableRenamedNodeLoop61Result::MlCallableRenamedNodeLoop61Done(inner_2, parameters_2) => {
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(vec![2f64, node, 2f64, -2f64], parameters_2), vec![-3f64]), self::callable_renamed_node(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, node, "body"), &(inner_2), false))
                                                        }
                                                    }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let plan_8 =
                            vec![2f64, node, self::read_required_number_fact(counts, node)];
                        {
                            let at_3 = self::read_required_number_fact(starts, node);
                            {
                                self::ml_callable_renamed_node_loop62(
                                    forms, texts, starts, counts, labels, targets, node, bindings,
                                    plan_8, at_3,
                                )
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
pub fn ml_read_type_separator_positions_loop1(
    units: &[f64],
    mut separator: f64,
    mut depth: f64,
    mut positions: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (units.len() as f64) {
            {
                let unit = self::read_required_number_fact(units, at);
                {
                    let depth_3 = if (((unit == 123f64) || (unit == 91f64)) || (unit == 60f64))
                        || (unit == 40f64)
                    {
                        { depth + 1f64 }
                    } else {
                        depth
                    };
                    {
                        let previous = if at > 0f64 {
                            self::read_required_number_fact(units, (at - 1f64))
                        } else {
                            -1f64
                        };
                        {
                            let depth_5 = if ((((unit == 125f64) || (unit == 93f64))
                                || (unit == 62f64))
                                || (unit == 41f64))
                                && !((unit == 62f64) && (previous == 61f64))
                            {
                                { depth_3 - 1f64 }
                            } else {
                                depth_3
                            };
                            {
                                let positions_3 = if (depth_5 == 0f64)
                                    && ((unit).partial_cmp(&(separator))
                                        == Some(std::cmp::Ordering::Equal))
                                {
                                    {
                                        {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![at], positions);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        }
                                    }
                                } else {
                                    positions
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (depth, positions, at) = (depth_5, positions_3, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { positions };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_type_trim_start_loop2(units: &[f64], mut end: f64, mut at: f64) -> f64 {
    loop {
        if (at < end)
            && self::accept_callable_type_whitespace(self::read_required_number_fact(units, at))
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

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_type_trim_end_loop3(units: &[f64], mut start: f64, mut at: f64) -> f64 {
    loop {
        if (at > start)
            && self::accept_callable_type_whitespace(self::read_required_number_fact(
                units,
                (at - 1f64),
            ))
        {
            {
                let at_2 = at - 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_type_parameter_segments_loop4(
    mut start: f64,
    part: &[f64],
    colons: &[f64],
    mut from: f64,
    mut at: f64,
    mut segments: Vec<f64>,
) -> Vec<f64> {
    loop {
        if at <= (colons.len() as f64) {
            {
                let to = if at == (colons.len() as f64) {
                    part.len() as f64
                } else {
                    self::read_required_number_fact(colons, at)
                };
                {
                    let begin = self::callable_type_trim_start(part, from, to);
                    {
                        let segments_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![
                                    (start + begin),
                                    (start + self::callable_type_trim_end(part, begin, to)),
                                ],
                                segments,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let from_2 = to + 1f64;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (from, at, segments) = (from_2, at_2, segments_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { segments };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCallableTypeParametersLoop5Result {
    MlCallableTypeParametersLoop5Done(f64, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCallableTypeParametersJoin6 {
    MlCallableTypeParametersJoin6Next(f64, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_type_parameters_loop5(
    units: &[f64],
    mut start: f64,
    mut named: bool,
    part: &[f64],
    commas: &[f64],
    mut from: f64,
    mut at: f64,
    mut count: f64,
    mut plan: Vec<f64>,
) -> self::MlCallableTypeParametersLoop5Result {
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
                            let ml_s23 = if finish > begin {
                                {
                                    let count_2 = count + 1f64;
                                    {
                                        let plan_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                self::callable_type_parameter_segments(
                                                    units,
                                                    (start + begin),
                                                    (start + finish),
                                                    named,
                                                ),
                                                plan,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlCallableTypeParametersJoin6::MlCallableTypeParametersJoin6Next(count_2, plan_2)
                                    }
                                }
                            } else {
                                self::MlCallableTypeParametersJoin6::MlCallableTypeParametersJoin6Next(count, plan)
                            };
                            match ml_s23 {
                                self::MlCallableTypeParametersJoin6::MlCallableTypeParametersJoin6Next(count_3, plan_3) => {
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
            self::MlCallableTypeParametersLoop5Result::MlCallableTypeParametersLoop5Done(
                count, plan,
            )
        };
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_type_syntax_loop8(units: &[f64], prefix: &[f64], mut at: f64) -> f64 {
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
pub enum MlPlanCallableTypeSyntaxJoin9 {
    MlPlanCallableTypeSyntaxJoin9Next,
    MlPlanCallableTypeSyntaxJoin9Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableTypeSyntaxLoop10Result {
    MlPlanCallableTypeSyntaxLoop10Done,
    MlPlanCallableTypeSyntaxLoop10Return(Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_type_syntax_loop10(
    units: &[f64],
    mut open: f64,
    mut close: f64,
) -> self::MlPlanCallableTypeSyntaxLoop10Result {
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
                            self::MlPlanCallableTypeSyntaxLoop10Result::MlPlanCallableTypeSyntaxLoop10Return(self::ml_array::append(vec![1f64, if result < (units.len() as f64) {
                                result
                            } else {
                                (units.len() as f64) - 1f64
                            }, (units.len() as f64)], self::callable_type_parameters(units, (open + 1f64), close_2, false)))
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
            self::MlPlanCallableTypeSyntaxLoop10Result::MlPlanCallableTypeSyntaxLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableTypeSyntaxJoin7 {
    MlPlanCallableTypeSyntaxJoin7Next,
    MlPlanCallableTypeSyntaxJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanCallableTypeSyntaxJoin12 {
    MlPlanCallableTypeSyntaxJoin12Next(f64, f64),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_plan_callable_type_syntax_loop11(
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
                        let ml_s24 = if unit == 41f64 {
                            {
                                let depth_4 = depth_3 - 1f64;
                                if depth_4 == 0f64 {
                                    {
                                        let close_2 = at;
                                        self::MlPlanCallableTypeSyntaxJoin12::MlPlanCallableTypeSyntaxJoin12Next(depth_4, close_2)
                                    }
                                } else {
                                    self::MlPlanCallableTypeSyntaxJoin12::MlPlanCallableTypeSyntaxJoin12Next(depth_4, close)
                                }
                            }
                        } else {
                            self::MlPlanCallableTypeSyntaxJoin12::MlPlanCallableTypeSyntaxJoin12Next(
                                depth_3, close,
                            )
                        };
                        match ml_s24 {
                            self::MlPlanCallableTypeSyntaxJoin12::MlPlanCallableTypeSyntaxJoin12Next(depth_5, close_3) => {
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
pub enum MlCallableFieldLoop13Result {
    MlCallableFieldLoop13Done,
    MlCallableFieldLoop13Return(f64),
}

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
