// Generated from complete js/src/translation/callable-source-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCallableRenamingBindingLoop61Result;
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
pub fn source_ml_source1_temporary_callable_renaming_binding_loop61(
    texts: &[String],
    bindings: &[f64],
    mut name: f64,
    mut at: f64,
) -> self::MlSource1TemporaryCallableRenamingBindingLoop61Result {
    loop {
        return if at > 0f64 {
            {
                let at_2 = at - 3f64;
                if (self::callable_text(texts, self::read_required_number_fact(bindings, at_2)))
                    == (self::callable_text(texts, name))
                {
                    self::MlSource1TemporaryCallableRenamingBindingLoop61Result::MlSource1TemporaryCallableRenamingBindingLoop61Return(vec![self::read_required_number_fact(bindings, (at_2 + 1f64)), self::read_required_number_fact(bindings, (at_2 + 2f64))])
                } else {
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCallableRenamingBindingLoop61Result::MlSource1TemporaryCallableRenamingBindingLoop61Done
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
pub fn source_ml_source1_temporary_callable_renamed_node_loop62(
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
                    let source_ml_source2_concat_value189 = plan;
                    {
                        let source_ml_source2_concat_value190 = self::callable_renamed_node(
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
                                source_ml_source2_concat_value189,
                            ),
                            source_ml_source2_concat_value190,
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
pub enum MlSource1TemporaryCallableRenamedNodeLoop63Result {
    MlSource1TemporaryCallableRenamedNodeLoop63Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_callable_renamed_node_loop63(
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
) -> self::MlSource1TemporaryCallableRenamedNodeLoop63Result {
    loop {
        if at < (self::read_required_number_fact(counts, params)) {
            {
                let parameter = self::callable_element(starts, counts, targets, params, at);
                {
                    let name =
                        self::callable_field(starts, counts, labels, targets, parameter, "name");
                    {
                        let inner_2 = {
                            let source_ml_source2_concat_value191 = inner;
                            {
                                let source_ml_source2_concat_value192 = vec![name, name, 0f64];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value191,
                                    ),
                                    source_ml_source2_concat_value192,
                                )
                            }
                        };
                        {
                            let value = self::callable_field(
                                starts, counts, labels, targets, parameter, "default",
                            );
                            {
                                let parameters_2 = {
                                    let source_ml_source2_concat_value195 = parameters;
                                    {
                                        let source_ml_source2_concat_value196 =
                                            if self::callable_truthy(forms, texts, value) {
                                                {
                                                    let source_ml_source2_concat_value193 =
                                                        vec![2f64, parameter, 1f64, -6f64];
                                                    {
                                                        let source_ml_source2_concat_value194 =
                                                            self::callable_renamed_node(
                                                                forms, texts, starts, counts,
                                                                labels, targets, value, bindings,
                                                                false,
                                                            );
                                                        self::ml_array::append(
                                                            self::ml_array::append(
                                                                Vec::<f64>::new(),
                                                                source_ml_source2_concat_value193,
                                                            ),
                                                            source_ml_source2_concat_value194,
                                                        )
                                                    }
                                                }
                                            } else {
                                                vec![2f64, parameter, 0f64]
                                            };
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value195,
                                            ),
                                            source_ml_source2_concat_value196,
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
            self::MlSource1TemporaryCallableRenamedNodeLoop63Result::MlSource1TemporaryCallableRenamedNodeLoop63Done(inner, parameters)
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
pub fn source_ml_source1_temporary_callable_renamed_node_loop64(
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
                            let source_ml_source2_concat_value197 = plan;
                            {
                                let source_ml_source2_concat_value198 = vec![at];
                                {
                                    let source_ml_source2_concat_value199 = if ((label == "span")
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
                                                source_ml_source2_concat_value197,
                                            ),
                                            source_ml_source2_concat_value198,
                                        ),
                                        source_ml_source2_concat_value199,
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
