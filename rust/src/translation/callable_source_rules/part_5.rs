// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableRenamedNodeLoop68Result;
use super::accept_callable_type_whitespace;
use super::callable_element;
use super::callable_field;
use super::callable_renaming_binding;
use super::callable_tag;
use super::callable_truthy;
use super::callable_type_parameter_segments;
use super::callable_type_parameters;
use super::callable_type_trim_end;
use super::callable_type_trim_start;
use super::ml_array;
use super::read_callable_source_pattern_names;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_renamed_node_loop67;
use super::source_ml_source1_temporary_callable_renamed_node_loop68;
use super::source_ml_source1_temporary_callable_renamed_node_loop69;

/// Execute the checked shared source algorithm.
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
                    self::source_ml_source1_temporary_callable_renamed_node_loop67(
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
                                    {
                                        let source_ml_source2_concat_value83 =
                                            vec![2f64, -1f64, 3f64, -7f64, 3f64, 0f64, -4f64];
                                        {
                                            let source_ml_source2_concat_value84 = renamed;
                                            {
                                                let source_ml_source2_concat_value85 =
                                                    vec![-8f64, 0f64, span];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value83,
                                                        ),
                                                        source_ml_source2_concat_value84,
                                                    ),
                                                    source_ml_source2_concat_value85,
                                                )
                                            }
                                        }
                                    }
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
                                            let plan_3 = {
                                                let source_ml_source2_concat_value86 = vec![
                                                    2f64,
                                                    node,
                                                    ((1f64 + if has_value { 1f64 } else { 0f64 })
                                                        + if has_body { 1f64 } else { 0f64 }),
                                                    -4f64,
                                                ];
                                                {
                                                    let source_ml_source2_concat_value87 =
                                                        self::callable_renamed_node(
                                                            forms, texts, starts, counts, labels,
                                                            targets, cell, bindings, true,
                                                        );
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value86,
                                                        ),
                                                        source_ml_source2_concat_value87,
                                                    )
                                                }
                                            };
                                            {
                                                let plan_5 = if has_value {
                                                    {
                                                        {
                                                            let source_ml_source2_concat_value88 =
                                                                plan_3;
                                                            {
                                                                let source_ml_source2_concat_value89 =
                                                                    vec![-5f64];
                                                                {
                                                                    let source_ml_source2_concat_value90 =
                                                                        self::callable_renamed_node(
                                                                            forms, texts, starts,
                                                                            counts, labels,
                                                                            targets, value,
                                                                            bindings, false,
                                                                        );
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value88), source_ml_source2_concat_value89), source_ml_source2_concat_value90)
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    plan_3
                                                };
                                                {
                                                    if has_body {
                                                        {
                                                            {
                                                                let source_ml_source2_concat_value91 =
                                                                    plan_5;
                                                                {
                                                                    let source_ml_source2_concat_value92 =
                                                                        vec![-3f64];
                                                                    {
                                                                        let source_ml_source2_concat_value93 = self::callable_renamed_node(forms, texts, starts, counts, labels, targets, body, bindings, false);
                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value91), source_ml_source2_concat_value92), source_ml_source2_concat_value93)
                                                                    }
                                                                }
                                                            }
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
                                        let source_ml_source2_s94 = self::source_ml_source1_temporary_callable_renamed_node_loop68(forms, texts, starts, counts, labels, targets, bindings, params, inner, parameters, at_2);
                                        match source_ml_source2_s94 {
                                                        self::MlSource1TemporaryCallableRenamedNodeLoop68Result::MlSource1TemporaryCallableRenamedNodeLoop68Done(inner_2, parameters_2) => {
                                                            {
                                                                let source_ml_source2_concat_value95 = vec![2f64, node, 2f64, -2f64];
                                                                {
                                                                    let source_ml_source2_concat_value96 = parameters_2;
                                                                    {
                                                                        let source_ml_source2_concat_value97 = vec![-3f64];
                                                                        {
                                                                            let source_ml_source2_concat_value98 = self::callable_renamed_node(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, node, "body"), &(inner_2), false);
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value95), source_ml_source2_concat_value96), source_ml_source2_concat_value97), source_ml_source2_concat_value98)
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
                    {
                        let plan_8 =
                            vec![2f64, node, self::read_required_number_fact(counts, node)];
                        {
                            let at_3 = self::read_required_number_fact(starts, node);
                            {
                                self::source_ml_source1_temporary_callable_renamed_node_loop69(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_type_separator_positions_loop1(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_type_trim_start_loop2(
    units: &[f64],
    mut end: f64,
    mut at: f64,
) -> f64 {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_type_trim_end_loop3(
    units: &[f64],
    mut start: f64,
    mut at: f64,
) -> f64 {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_type_parameter_segments_loop4(
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
                            let source_ml_source2_concat_value99 = segments;
                            {
                                let source_ml_source2_concat_value100 = vec![
                                    (start + begin),
                                    (start + self::callable_type_trim_end(part, begin, to)),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value99,
                                    ),
                                    source_ml_source2_concat_value100,
                                )
                            }
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
pub enum MlSource1TemporaryCallableTypeParametersLoop5Result {
    MlSource1TemporaryCallableTypeParametersLoop5Done(f64, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCallableTypeParametersJoin6 {
    MlSource1TemporaryCallableTypeParametersJoin6Next(f64, Vec<f64>),
}

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
                            let source_ml_source2_s103 = if finish > begin {
                                {
                                    let count_2 = count + 1f64;
                                    {
                                        let plan_2 = {
                                            let source_ml_source2_concat_value104 = plan;
                                            {
                                                let source_ml_source2_concat_value105 =
                                                    self::callable_type_parameter_segments(
                                                        units,
                                                        (start + begin),
                                                        (start + finish),
                                                        named,
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value104,
                                                    ),
                                                    source_ml_source2_concat_value105,
                                                )
                                            }
                                        };
                                        self::MlSource1TemporaryCallableTypeParametersJoin6::MlSource1TemporaryCallableTypeParametersJoin6Next(count_2, plan_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryCallableTypeParametersJoin6::MlSource1TemporaryCallableTypeParametersJoin6Next(count, plan)
                            };
                            match source_ml_source2_s103 {
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
                                let source_ml_source2_concat_value106 = vec![1f64, if result < (units.len() as f64) {
                                    result
                                } else {
                                    (units.len() as f64) - 1f64
                                }, (units.len() as f64)];
                                {
                                    let source_ml_source2_concat_value107 = self::callable_type_parameters(units, (open + 1f64), close_2, false);
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value106), source_ml_source2_concat_value107)
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
                        let source_ml_source2_s108 = if unit == 41f64 {
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
                        match source_ml_source2_s108 {
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
                    let source_ml_source2_concat_value109 = names;
                    {
                        let source_ml_source2_concat_value110 = vec![self::callable_tag(
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
                                source_ml_source2_concat_value109,
                            ),
                            source_ml_source2_concat_value110,
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
pub fn source_ml_source1_temporary_read_callable_source_pattern_names_loop16(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut names: Vec<String>,
    mut args: f64,
    mut at: f64,
) -> Vec<String> {
    loop {
        if (args >= 0f64) && (at < (self::read_required_number_fact(counts, args))) {
            {
                let names_2 = {
                    let source_ml_source2_concat_value111 = names;
                    {
                        let source_ml_source2_concat_value112 =
                            self::read_callable_source_pattern_names(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::callable_element(starts, counts, targets, args, at),
                            );
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<String>::new(),
                                source_ml_source2_concat_value111,
                            ),
                            source_ml_source2_concat_value112,
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
