// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableCellObservationsLoop57Result;
use super::MlSource1TemporaryCallableRenamedNodeLoop63Result;
use super::MlSource1TemporaryCallableRenamingBindingLoop61Result;
use super::accept_callable_type_whitespace;
use super::callable_element;
use super::callable_field;
use super::callable_tag;
use super::callable_truthy;
use super::callable_type_trim_end;
use super::callable_type_trim_start;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_callable_cell_observations_loop57;
use super::source_ml_source1_temporary_callable_cell_observations_loop58;
use super::source_ml_source1_temporary_callable_cell_observations_loop59;
use super::source_ml_source1_temporary_callable_renamed_node_loop62;
use super::source_ml_source1_temporary_callable_renamed_node_loop63;
use super::source_ml_source1_temporary_callable_renamed_node_loop64;
use super::source_ml_source1_temporary_callable_renaming_binding_loop61;
use super::source_ml_source1_temporary_plan_callable_source_renaming_loop60;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_cell_observations(
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
    } else {
        {
            let events = Vec::<f64>::new();
            {
                let events_3 =
                    if (self::callable_tag(texts, starts, counts, labels, targets, node, "s"))
                        == "assign"
                    {
                        {
                            vec![
                                1f64,
                                self::callable_field(starts, counts, labels, targets, node, "name"),
                            ]
                        }
                    } else {
                        events
                    };
                {
                    let events_5 =
                        if (self::callable_tag(texts, starts, counts, labels, targets, node, "k"))
                            == "cellWrite"
                        {
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
                                            {
                                                {
                                                    let source_ml_source2_concat_value76 = events_3;
                                                    {
                                                        let source_ml_source2_concat_value77 =
                                                            vec![0f64, name, 1f64, name];
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<f64>::new(),
                                                                source_ml_source2_concat_value76,
                                                            ),
                                                            source_ml_source2_concat_value77,
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        events_3
                                    }
                                }
                            }
                        } else {
                            events_3
                        };
                    {
                        let events_8 = if (self::callable_tag(
                            texts, starts, counts, labels, targets, node, "k",
                        )) == "lambda"
                        {
                            {
                                let captured = self::callable_field(
                                    starts, counts, labels, targets, node, "captures",
                                );
                                {
                                    let assigned = self::callable_field(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        "assignedCaptures",
                                    );
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s78 = self::source_ml_source1_temporary_callable_cell_observations_loop57(starts, counts, targets, events_5, captured, at);
                                            match source_ml_source2_s78 {
                                                self::MlSource1TemporaryCallableCellObservationsLoop57Result::MlSource1TemporaryCallableCellObservationsLoop57Done(events_6, at_2) => {
                                                    {
                                                        let at_3 = 0f64;
                                                        {
                                                            self::source_ml_source1_temporary_callable_cell_observations_loop58(starts, counts, targets, events_6, assigned, at_3)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            events_5
                        };
                        {
                            let at_4 = self::read_required_number_fact(starts, node);
                            {
                                self::source_ml_source1_temporary_callable_cell_observations_loop59(
                                    forms, texts, starts, counts, labels, targets, node, events_8,
                                    at_4,
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
pub fn read_callable_cell_lowering(operation: &str, reference: bool) -> String {
    if reference {
        if operation == "read" {
            String::from("cellRead")
        } else if operation == "assign" {
            String::from("cellWrite")
        } else if operation == "declare" {
            String::from("cellAllocate")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_cell_parameter_name(source: &str, reference: bool) -> String {
    if reference {
        format!("ml_cell_{}", source.to_owned())
    } else {
        source.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_cell_error(kind: &str, operation: &str) -> String {
    if kind == "cell" {
        String::new()
    } else if operation == "cellRead" {
        String::from("reading a value requires a reference cell")
    } else {
        String::from("writing a value requires a reference cell")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_cell_element(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    expected: f64,
) -> Vec<f64> {
    {
        let type_ = self::callable_field(starts, counts, labels, targets, node, "type");
        {
            let element = self::callable_field(starts, counts, labels, targets, type_, "element");
            if self::callable_truthy(forms, texts, element) {
                vec![element, 1f64]
            } else {
                vec![
                    self::callable_field(starts, counts, labels, targets, expected, "element"),
                    0f64,
                ]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_cell_read_inference(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    cell: f64,
    resolved: f64,
) -> Vec<f64> {
    if (self::callable_tag(texts, starts, counts, labels, targets, resolved, "kind")) == "cell" {
        vec![self::callable_field(
            starts, counts, labels, targets, resolved, "element",
        )]
    } else {
        vec![-1f64, cell]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_callable_cell_option_binding(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    resolved: f64,
    value: f64,
) -> Vec<f64> {
    {
        let element = self::callable_field(starts, counts, labels, targets, resolved, "element");
        if (((self::callable_tag(texts, starts, counts, labels, targets, resolved, "kind"))
            != "cell")
            || ((self::callable_tag(texts, starts, counts, labels, targets, element, "kind"))
                != "var"))
            || ((self::callable_tag(texts, starts, counts, labels, targets, value, "kind"))
                != "option")
        {
            Vec::<f64>::new()
        } else {
            vec![
                self::callable_field(starts, counts, labels, targets, element, "id"),
                value,
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_callable_cell_operation(
    format: &str,
    operation: &str,
    cell: &str,
    value: &str,
    body: &str,
    cell_name: &str,
    value_name: &str,
) -> String {
    if format == "JavaScript" {
        if operation == "cellAllocate" {
            format!("({{ value: {} }})", value.to_owned())
        } else if operation == "cellRead" {
            format!("({}).value", cell.to_owned())
        } else {
            format!(
                "(({}).value = {}, {})",
                cell.to_owned(),
                value.to_owned(),
                body.to_owned()
            )
        }
    } else if operation == "cellAllocate" {
        format!(
            "std::rc::Rc::new(std::cell::RefCell::new({}))",
            value.to_owned()
        )
    } else if operation == "cellRead" {
        format!(
            "{{ let ml_cell_value = {}.borrow().clone(); ml_cell_value }}",
            cell.to_owned()
        )
    } else {
        format!(
            "{{ let {} = {}; let {} = {}; *{}.borrow_mut() = {}; {} }}",
            cell_name.to_owned(),
            cell.to_owned(),
            value_name.to_owned(),
            value.to_owned(),
            cell_name.to_owned(),
            value_name.to_owned(),
            body.to_owned()
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_reference_obligation(temporal: bool, local: bool) -> String {
    if temporal {
        String::from(
            "the binding is in its temporal dead zone, where reading it throws a ReferenceError",
        )
    } else if local {
        String::new()
    } else {
        String::from("functions are only portable when called")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_reference_construct(name: &str, temporal: bool) -> String {
    if temporal {
        format!("use of {} before its declaration", name.to_owned())
    } else {
        format!("function value {}", name.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_renaming_field(field: f64) -> String {
    if field == 0f64 {
        String::from("path")
    } else if field == 1f64 {
        String::from("params")
    } else if field == 2f64 {
        String::from("body")
    } else if field == 3f64 {
        String::from("cell")
    } else if field == 4f64 {
        String::from("value")
    } else if field == 5f64 {
        String::from("default")
    } else if field == 6f64 {
        String::from("k")
    } else if field == 7f64 {
        String::from("span")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_renaming_constant(constant: f64) -> String {
    if constant == 0f64 {
        String::from("cellRead")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
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
                let bindings_2 =
                    self::source_ml_source1_temporary_plan_callable_source_renaming_loop60(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn callable_renaming_binding(texts: &[String], bindings: &[f64], name: f64) -> Vec<f64> {
    {
        let at = bindings.len() as f64;
        {
            let source_ml_source2_s79 =
                self::source_ml_source1_temporary_callable_renaming_binding_loop61(
                    texts, bindings, name, at,
                );
            match source_ml_source2_s79 {
                self::MlSource1TemporaryCallableRenamingBindingLoop61Result::MlSource1TemporaryCallableRenamingBindingLoop61Done => {
                    vec![name, 0f64]
                }
                self::MlSource1TemporaryCallableRenamingBindingLoop61Result::MlSource1TemporaryCallableRenamingBindingLoop61Return(source_ml_source1_temporary_result61) => {
                    source_ml_source1_temporary_result61
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
                    self::source_ml_source1_temporary_callable_renamed_node_loop62(
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
                                        let source_ml_source2_concat_value80 =
                                            vec![2f64, -1f64, 3f64, -7f64, 3f64, 0f64, -4f64];
                                        {
                                            let source_ml_source2_concat_value81 = renamed;
                                            {
                                                let source_ml_source2_concat_value82 =
                                                    vec![-8f64, 0f64, span];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value80,
                                                        ),
                                                        source_ml_source2_concat_value81,
                                                    ),
                                                    source_ml_source2_concat_value82,
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
                                                let source_ml_source2_concat_value83 = vec![
                                                    2f64,
                                                    node,
                                                    ((1f64 + if has_value { 1f64 } else { 0f64 })
                                                        + if has_body { 1f64 } else { 0f64 }),
                                                    -4f64,
                                                ];
                                                {
                                                    let source_ml_source2_concat_value84 =
                                                        self::callable_renamed_node(
                                                            forms, texts, starts, counts, labels,
                                                            targets, cell, bindings, true,
                                                        );
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value83,
                                                        ),
                                                        source_ml_source2_concat_value84,
                                                    )
                                                }
                                            };
                                            {
                                                let plan_5 = if has_value {
                                                    {
                                                        {
                                                            let source_ml_source2_concat_value85 =
                                                                plan_3;
                                                            {
                                                                let source_ml_source2_concat_value86 =
                                                                    vec![-5f64];
                                                                {
                                                                    let source_ml_source2_concat_value87 =
                                                                        self::callable_renamed_node(
                                                                            forms, texts, starts,
                                                                            counts, labels,
                                                                            targets, value,
                                                                            bindings, false,
                                                                        );
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value85), source_ml_source2_concat_value86), source_ml_source2_concat_value87)
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
                                                                let source_ml_source2_concat_value88 =
                                                                    plan_5;
                                                                {
                                                                    let source_ml_source2_concat_value89 =
                                                                        vec![-3f64];
                                                                    {
                                                                        let source_ml_source2_concat_value90 = self::callable_renamed_node(forms, texts, starts, counts, labels, targets, body, bindings, false);
                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value88), source_ml_source2_concat_value89), source_ml_source2_concat_value90)
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
                                        let source_ml_source2_s91 = self::source_ml_source1_temporary_callable_renamed_node_loop63(forms, texts, starts, counts, labels, targets, bindings, params, inner, parameters, at_2);
                                        match source_ml_source2_s91 {
                                                        self::MlSource1TemporaryCallableRenamedNodeLoop63Result::MlSource1TemporaryCallableRenamedNodeLoop63Done(inner_2, parameters_2) => {
                                                            {
                                                                let source_ml_source2_concat_value92 = vec![2f64, node, 2f64, -2f64];
                                                                {
                                                                    let source_ml_source2_concat_value93 = parameters_2;
                                                                    {
                                                                        let source_ml_source2_concat_value94 = vec![-3f64];
                                                                        {
                                                                            let source_ml_source2_concat_value95 = self::callable_renamed_node(forms, texts, starts, counts, labels, targets, self::callable_field(starts, counts, labels, targets, node, "body"), &(inner_2), false);
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value92), source_ml_source2_concat_value93), source_ml_source2_concat_value94), source_ml_source2_concat_value95)
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
                                self::source_ml_source1_temporary_callable_renamed_node_loop64(
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
                            let source_ml_source2_concat_value96 = segments;
                            {
                                let source_ml_source2_concat_value97 = vec![
                                    (start + begin),
                                    (start + self::callable_type_trim_end(part, begin, to)),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value96,
                                    ),
                                    source_ml_source2_concat_value97,
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
