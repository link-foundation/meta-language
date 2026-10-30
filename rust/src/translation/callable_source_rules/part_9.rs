// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableRenamingBindingLoop66Result;
use super::callable_element;
use super::callable_field;
use super::callable_renamed_node;
use super::callable_text;
use super::callable_truthy;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_callable_renaming_binding_loop66(
    texts: &[String],
    bindings: &[f64],
    mut name: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCallableRenamingBindingLoop66Result {
    loop {
        return if at > 0f64 {
            {
                let at_2 = at - 3f64;
                if (self::callable_text(texts, self::read_required_number_fact(bindings, at_2)))
                    == (self::callable_text(texts, name))
                {
                    self::MlSource1TemporaryCallableRenamingBindingLoop66Result::MlSource1TemporaryCallableRenamingBindingLoop66Return(vec![self::read_required_number_fact(bindings, (at_2 + 1f64)), self::read_required_number_fact(bindings, (at_2 + 2f64))])
                } else {
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCallableRenamingBindingLoop66Result::MlSource1TemporaryCallableRenamingBindingLoop66Done
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
pub fn source_ml_source1_temporary_callable_renamed_node_loop67(
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
                    let source_ml_source2_concat_value214 = plan;
                    {
                        let source_ml_source2_concat_value215 = self::callable_renamed_node(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::callable_element(starts, counts, targets, node, at),
                            bindings,
                            false,
                        );
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value214,
                            ),
                            source_ml_source2_concat_value215,
                        )
                    }
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
pub enum MlSource1TemporaryCallableRenamedNodeLoop68Result {
    MlSource1TemporaryCallableRenamedNodeLoop68Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_renamed_node_loop68(
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
) -> self::MlSource1TemporaryCallableRenamedNodeLoop68Result {
    loop {
        if at < (self::read_required_number_fact(counts, params)) {
            {
                let parameter = self::callable_element(starts, counts, targets, params, at);
                {
                    let name =
                        self::callable_field(starts, counts, labels, targets, parameter, "name");
                    {
                        let inner_2 = {
                            let source_ml_source2_concat_value216 = inner;
                            {
                                let source_ml_source2_concat_value217 = vec![name, name, 0f64];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value216,
                                    ),
                                    source_ml_source2_concat_value217,
                                )
                            }
                        };
                        {
                            let value = self::callable_field(
                                starts, counts, labels, targets, parameter, "default",
                            );
                            {
                                let parameters_2 = {
                                    let source_ml_source2_concat_value220 = parameters;
                                    {
                                        let source_ml_source2_concat_value221 =
                                            if self::callable_truthy(forms, texts, value) {
                                                {
                                                    let source_ml_source2_concat_value218 =
                                                        vec![2f64, parameter, 1f64, -6f64];
                                                    {
                                                        let source_ml_source2_concat_value219 =
                                                            self::callable_renamed_node(
                                                                forms, texts, starts, counts,
                                                                labels, targets, value, bindings,
                                                                false,
                                                            );
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<f64>::new(),
                                                                source_ml_source2_concat_value218,
                                                            ),
                                                            source_ml_source2_concat_value219,
                                                        )
                                                    }
                                                }
                                            } else {
                                                vec![2f64, parameter, 0f64]
                                            };
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value220,
                                            ),
                                            source_ml_source2_concat_value221,
                                        )
                                    }
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
            self::MlSource1TemporaryCallableRenamedNodeLoop68Result::MlSource1TemporaryCallableRenamedNodeLoop68Done(inner, parameters)
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
pub fn source_ml_source1_temporary_callable_renamed_node_loop69(
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
                        let plan_2 = {
                            let source_ml_source2_concat_value222 = plan;
                            {
                                let source_ml_source2_concat_value223 = vec![at];
                                {
                                    let source_ml_source2_concat_value224 = if ((label == "span")
                                        || (label == "type"))
                                        || (label == "data")
                                    {
                                        vec![0f64, value]
                                    } else {
                                        self::callable_renamed_node(
                                            forms, texts, starts, counts, labels, targets, value,
                                            bindings, false,
                                        )
                                    };
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value222,
                                            ),
                                            source_ml_source2_concat_value223,
                                        ),
                                        source_ml_source2_concat_value224,
                                    )
                                }
                            }
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
            }
        }
        return { plan };
    }
}
