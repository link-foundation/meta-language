// Generated from complete callable-source-rules.js through the checked translator.
// Do not edit by hand.
use super::callable_cell_observations;
use super::callable_element;
use super::callable_field;
use super::callable_renamed_node;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_cell_observations_loop57(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    mut events: Vec<f64>,
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
                    let events_3 = if ((((label != "span") && (label != "type"))
                        && (label != "data"))
                        && (label != "captures"))
                        && (label != "assignedCaptures")
                    {
                        {
                            {
                                let (ml_append_right, ml_append_left) = (
                                    self::callable_cell_observations(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        self::read_required_number_fact(targets, at),
                                    ),
                                    events,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        events
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (events, at) = (events_3, at_2);
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_source_renaming_loop58(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut scope: f64,
    mut bindings: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (scope >= 0f64) && (at < (self::read_required_number_fact(counts, scope))) {
            {
                let binding = self::callable_element(starts, counts, targets, scope, at);
                {
                    let source =
                        self::callable_field(starts, counts, labels, targets, binding, "source");
                    {
                        let target =
                            self::callable_field(starts, counts, labels, targets, binding, "ir");
                        {
                            let reference = self::callable_truthy(
                                forms,
                                texts,
                                self::callable_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    binding,
                                    "reference",
                                ),
                            );
                            {
                                let bindings_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![source, target, if reference { 1f64 } else { 0f64 }],
                                        bindings,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (bindings, at) = (bindings_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { bindings };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCallableRenamingBindingLoop59Result {
    MlCallableRenamingBindingLoop59Done,
    MlCallableRenamingBindingLoop59Return(Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn ml_callable_renaming_binding_loop59(
    texts: &[String],
    bindings: &[f64],
    mut name: f64,
    mut at: f64,
) -> self::MlCallableRenamingBindingLoop59Result {
    loop {
        return if at > 0f64 {
            {
                let at_2 = at - 3f64;
                if (self::callable_text(texts, self::read_required_number_fact(bindings, at_2)))
                    == (self::callable_text(texts, name))
                {
                    self::MlCallableRenamingBindingLoop59Result::MlCallableRenamingBindingLoop59Return(vec![self::read_required_number_fact(bindings, (at_2 + 1f64)), self::read_required_number_fact(bindings, (at_2 + 2f64))])
                } else {
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlCallableRenamingBindingLoop59Result::MlCallableRenamingBindingLoop59Done
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
pub fn ml_callable_renamed_node_loop60(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    bindings: &[f64],
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (self::read_required_number_fact(counts, node)) {
            {
                let plan_2 = {
                    let (ml_append_right, ml_append_left) = (
                        self::callable_renamed_node(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_element(starts, counts, targets, node, at),
                            bindings,
                            false,
                        ),
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
        return { plan };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCallableRenamedNodeLoop61Result {
    MlCallableRenamedNodeLoop61Done(Vec<f64>, Vec<f64>),
}

/// Execute the complete checked source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_renamed_node_loop61(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bindings: &[f64],
    mut params: f64,
    mut inner: Vec<f64>,
    mut parameters: Vec<f64>,
    mut at: f64,
) -> self::MlCallableRenamedNodeLoop61Result {
    loop {
        if at < (self::read_required_number_fact(counts, params)) {
            {
                let parameter = self::callable_element(starts, counts, targets, params, at);
                {
                    let name =
                        self::callable_field(starts, counts, labels, targets, parameter, "name");
                    {
                        let inner_2 = {
                            let (ml_append_right, ml_append_left) = (vec![name, name, 0f64], inner);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let value = self::callable_field(
                                starts, counts, labels, targets, parameter, "default",
                            );
                            {
                                let parameters_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        if self::callable_truthy(forms, texts, value) {
                                            self::ml_array::append(
                                                vec![2f64, parameter, 1f64, -6f64],
                                                self::callable_renamed_node(
                                                    forms, texts, starts, counts, labels, targets,
                                                    value, bindings, false,
                                                ),
                                            )
                                        } else {
                                            vec![2f64, parameter, 0f64]
                                        },
                                        parameters,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (inner, parameters, at) = (inner_2, parameters_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlCallableRenamedNodeLoop61Result::MlCallableRenamedNodeLoop61Done(
                inner, parameters,
            )
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
pub fn ml_callable_renamed_node_loop62(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut node: f64,
    bindings: &[f64],
    mut plan: Vec<f64>,
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
                    let value = self::read_required_number_fact(targets, at);
                    {
                        let plan_2 = self::ml_array::append(
                            {
                                let (ml_append_right, ml_append_left) = (vec![at], plan);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            },
                            if ((label == "span") || (label == "type")) || (label == "data") {
                                vec![0f64, value]
                            } else {
                                self::callable_renamed_node(
                                    forms, texts, starts, counts, labels, targets, value, bindings,
                                    false,
                                )
                            },
                        );
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
        }
        return { plan };
    }
}
