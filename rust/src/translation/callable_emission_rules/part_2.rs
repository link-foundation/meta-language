// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::accept_callable_creation_effect_traversal;
use super::accept_callable_effect_field;
use super::callable_emission_field;
use super::callable_emission_has_name;
use super::callable_emission_text;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::require_callable_output_effect;
use super::source_ml_temporary_callable_emission_obligations_loop15;
use super::source_ml_temporary_plan_callable_output_obligations_loop17;
use super::source_ml_temporary_read_callable_effectful_declarations_loop12;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_callable_eager_effects(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    names: &[String],
) -> bool {
    if node < 0f64 {
        false
    } else {
        {
            let form = self::read_required_text_fact(forms, node);
            if (form != "object") && (form != "array") {
                false
            } else {
                {
                    let kind = self::callable_emission_text(
                        texts, starts, counts, labels, targets, node, "k",
                    );
                    if (self::accept_callable_creation_effect_traversal(&(kind))) {
                        if (self::require_callable_output_effect(
                            &(kind),
                            &(self::callable_emission_text(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                "semantics",
                            )),
                            &(self::callable_emission_text(
                                texts, starts, counts, labels, targets, node, "flavor",
                            )),
                            &(self::callable_emission_text(
                                texts, starts, counts, labels, targets, node, "byZero",
                            )),
                        )) || ((kind == "call")
                            && self::callable_emission_has_name(
                                names,
                                &(self::callable_emission_text(
                                    texts, starts, counts, labels, targets, node, "fn",
                                )),
                            ))
                        {
                            true
                        } else {
                            {
                                let source_ml_source1_s3 = if kind == "apply" {
                                    {
                                        let fn_ = self::callable_emission_field(
                                            starts, counts, labels, targets, node, "fn",
                                        );
                                        {
                                            let target = self::callable_emission_text(
                                                texts, starts, counts, labels, targets, fn_, "k",
                                            );
                                            if ((target == "functionRef")
                                                && self::callable_emission_has_name(
                                                    names,
                                                    &(self::callable_emission_text(
                                                        texts, starts, counts, labels, targets,
                                                        fn_, "fn",
                                                    )),
                                                ))
                                                || ((target == "lambda")
                                                    && self::read_callable_eager_effects(
                                                        forms,
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        self::callable_emission_field(
                                                            starts, counts, labels, targets, fn_,
                                                            "body",
                                                        ),
                                                        names,
                                                    ))
                                            {
                                                self::MlTemporaryReadCallableEagerEffectsJoin10::MlTemporaryReadCallableEagerEffectsJoin10Return(true)
                                            } else {
                                                self::MlTemporaryReadCallableEagerEffectsJoin10::MlTemporaryReadCallableEagerEffectsJoin10Next
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadCallableEagerEffectsJoin10::MlTemporaryReadCallableEagerEffectsJoin10Next
                                };
                                match source_ml_source1_s3 {
                                        self::MlTemporaryReadCallableEagerEffectsJoin10::MlTemporaryReadCallableEagerEffectsJoin10Next => {
                                            {
                                                let at = self::read_required_number_fact(starts, node);
                                                {
                                                    let end = at + self::read_required_number_fact(counts, node);
                                                    {
                                                        let source_ml_source1_s4 = self::source_ml_temporary_read_callable_eager_effects_loop11(forms, texts, starts, counts, labels, targets, names, at, end);
                                                        match source_ml_source1_s4 {
                                                            self::MlTemporaryReadCallableEagerEffectsLoop11Result::MlTemporaryReadCallableEagerEffectsLoop11Done => {
                                                                false
                                                            }
                                                            self::MlTemporaryReadCallableEagerEffectsLoop11Result::MlTemporaryReadCallableEagerEffectsLoop11Return(source_ml_temporary_result11) => {
                                                                source_ml_temporary_result11
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryReadCallableEagerEffectsJoin10::MlTemporaryReadCallableEagerEffectsJoin10Return(source_ml_temporary_result11_2) => {
                                            source_ml_temporary_result11_2
                                        }
                                    }
                            }
                        }
                    } else {
                        false
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_effectful_declarations(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    declarations: f64,
) -> Vec<f64> {
    {
        let selected = Vec::<f64>::new();
        {
            let names = Vec::<String>::new();
            {
                let grew = true;
                {
                    self::source_ml_temporary_read_callable_effectful_declarations_loop12(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        declarations,
                        selected,
                        names,
                        grew,
                    )
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
pub fn callable_emission_obligations(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    names: &[String],
) -> Vec<f64> {
    if node < 0f64 {
        Vec::<f64>::new()
    } else {
        {
            let form = self::read_required_text_fact(forms, node);
            if (form != "object") && (form != "array") {
                Vec::<f64>::new()
            } else {
                {
                    let kind = self::callable_emission_text(
                        texts, starts, counts, labels, targets, node, "k",
                    );
                    {
                        let selected = Vec::<f64>::new();
                        {
                            let selected_3 = if (kind == "lambda")
                                && self::read_callable_eager_effects(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::callable_emission_field(
                                        starts, counts, labels, targets, node, "body",
                                    ),
                                    names,
                                ) {
                                { vec![node] }
                            } else {
                                selected
                            };
                            {
                                let selected_5 = if (kind == "functionRef")
                                    && self::callable_emission_has_name(
                                        names,
                                        &(self::callable_emission_text(
                                            texts, starts, counts, labels, targets, node, "fn",
                                        )),
                                    ) {
                                    { vec![node] }
                                } else {
                                    selected_3
                                };
                                {
                                    let selected_9 = if kind == "apply" {
                                        {
                                            let fn_ = self::callable_emission_field(
                                                starts, counts, labels, targets, node, "fn",
                                            );
                                            {
                                                let target = self::callable_emission_text(
                                                    texts, starts, counts, labels, targets, fn_,
                                                    "k",
                                                );
                                                {
                                                    let selected_7 = if (target == "functionRef")
                                                        && self::callable_emission_has_name(
                                                            names,
                                                            &(self::callable_emission_text(
                                                                texts, starts, counts, labels,
                                                                targets, fn_, "fn",
                                                            )),
                                                        ) {
                                                        { vec![node] }
                                                    } else {
                                                        selected_5
                                                    };
                                                    if (target == "lambda")
                                                        && self::read_callable_eager_effects(
                                                            forms,
                                                            texts,
                                                            starts,
                                                            counts,
                                                            labels,
                                                            targets,
                                                            self::callable_emission_field(
                                                                starts, counts, labels, targets,
                                                                fn_, "body",
                                                            ),
                                                            names,
                                                        )
                                                    {
                                                        { vec![node] }
                                                    } else {
                                                        selected_7
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        selected_5
                                    };
                                    {
                                        let at = self::read_required_number_fact(starts, node);
                                        {
                                            let end =
                                                at + self::read_required_number_fact(counts, node);
                                            {
                                                self::source_ml_temporary_callable_emission_obligations_loop15(forms, texts, starts, counts, labels, targets, names, selected_9, at, end)
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_callable_output_obligations(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    node: f64,
    declarations: &[f64],
) -> Vec<f64> {
    {
        let names = Vec::<String>::new();
        {
            let source_ml_temporary_values16 = declarations.to_vec();
            {
                let source_ml_temporary_index16 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_temporary_plan_callable_output_obligations_loop17(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        names,
                        &(source_ml_temporary_values16),
                        source_ml_temporary_index16,
                    );
                    self::callable_emission_obligations(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(names_2),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_callable_output_target_refusal(target: &str, count: f64) -> String {
    if ((target == "Lean") || (target == "Rocq")) && (count > 0f64) {
        String::from(
            "function values that print or may abort require an output-threaded function type",
        )
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCallableEmissionFieldLoop1Result {
    MlTemporaryCallableEmissionFieldLoop1Done,
    MlTemporaryCallableEmissionFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_callable_emission_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut at: f64,
    mut end: f64,
) -> self::MlTemporaryCallableEmissionFieldLoop1Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlTemporaryCallableEmissionFieldLoop1Result::MlTemporaryCallableEmissionFieldLoop1Return(self::read_required_number_fact(targets, at))
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
            self::MlTemporaryCallableEmissionFieldLoop1Result::MlTemporaryCallableEmissionFieldLoop1Done
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
pub fn source_ml_temporary_plan_callable_capture_bindings_loop4(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    selected: &[f64],
    name: &str,
    mut position: f64,
    mut at: f64,
) -> f64 {
    loop {
        if at < (selected.len() as f64) {
            {
                let previous = self::read_required_number_fact(selected, at);
                {
                    let position_3 = if (self::callable_emission_text(
                        texts, starts, counts, labels, targets, previous, "name",
                    )) == name
                    {
                        { at }
                    } else {
                        position
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (position, at) = (position_3, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { position };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_callable_capture_bindings_loop3(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut selected: Vec<f64>,
    source_ml_temporary_values2: &[f64],
    mut source_ml_temporary_index2: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_temporary_index2
            < (self::ml::Big::from_u128(source_ml_temporary_values2.len() as u128))
        {
            {
                let reference = self::ml_array::at(
                    source_ml_temporary_values2,
                    source_ml_temporary_index2.to_index(),
                );
                {
                    let name = self::callable_emission_text(
                        texts, starts, counts, labels, targets, reference, "name",
                    );
                    {
                        let position = -1f64;
                        {
                            let at = 0f64;
                            {
                                let position_2 =
                                    self::source_ml_temporary_plan_callable_capture_bindings_loop4(
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        &(selected),
                                        &(name),
                                        position,
                                        at,
                                    );
                                if position_2 < 0f64 {
                                    {
                                        let selected_2 = {
                                            let source_ml_source1_concat_value5 = selected;
                                            {
                                                let source_ml_source1_concat_value6 =
                                                    vec![reference];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source1_concat_value5,
                                                    ),
                                                    source_ml_source1_concat_value6,
                                                )
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index2_2 =
                                                source_ml_temporary_index2
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (selected, source_ml_temporary_index2) =
                                                    (selected_2, source_ml_temporary_index2_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let selected_3 = {
                                            let source_ml_source1_concat_value9 = {
                                                let source_ml_source1_concat_value7 =
                                                    self::ml_array::slice(
                                                        &selected, 0f64, position_2,
                                                    );
                                                {
                                                    let source_ml_source1_concat_value8 =
                                                        vec![reference];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source1_concat_value7,
                                                        ),
                                                        source_ml_source1_concat_value8,
                                                    )
                                                }
                                            };
                                            {
                                                let source_ml_source1_concat_value10 =
                                                    self::ml_array::slice(
                                                        &selected,
                                                        (position_2 + 1f64),
                                                        f64::INFINITY,
                                                    );
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source1_concat_value9,
                                                    ),
                                                    source_ml_source1_concat_value10,
                                                )
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index2_3 =
                                                source_ml_temporary_index2
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (selected, source_ml_temporary_index2) =
                                                    (selected_3, source_ml_temporary_index2_3);
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
        return { selected };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Result {
    MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Done(Vec<String>, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanFormalReferenceStoreFunctionsJoin7 {
    MlTemporaryPlanFormalReferenceStoreFunctionsJoin7Next(Vec<String>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_reference_store_functions_loop6(
    callers: &[String],
    callees: &[String],
    mut selected: Vec<String>,
    mut changed: bool,
    mut position: f64,
) -> self::MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Result {
    loop {
        return if position < (callers.len() as f64) {
            {
                let caller = self::read_required_text_fact(callers, position);
                {
                    let source_ml_source1_s13 = if !(self::callable_emission_has_name(
                        &(selected),
                        &(caller),
                    )) && self::callable_emission_has_name(
                        &(selected),
                        &(self::read_required_text_fact(callees, position)),
                    ) {
                        {
                            let selected_2 = {
                                let source_ml_source1_concat_value14 = selected;
                                {
                                    let source_ml_source1_concat_value15 = vec![caller.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source1_concat_value14,
                                        ),
                                        source_ml_source1_concat_value15,
                                    )
                                }
                            };
                            {
                                let changed_2 = true;
                                self::MlTemporaryPlanFormalReferenceStoreFunctionsJoin7::MlTemporaryPlanFormalReferenceStoreFunctionsJoin7Next(selected_2, changed_2)
                            }
                        }
                    } else {
                        self::MlTemporaryPlanFormalReferenceStoreFunctionsJoin7::MlTemporaryPlanFormalReferenceStoreFunctionsJoin7Next(selected, changed)
                    };
                    match source_ml_source1_s13 {
                        self::MlTemporaryPlanFormalReferenceStoreFunctionsJoin7::MlTemporaryPlanFormalReferenceStoreFunctionsJoin7Next(selected_3, changed_3) => {
                            {
                                let position_2 = position + 1f64;
                                {
                                    (selected, changed, position) = (selected_3, changed_3, position_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Result::MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Done(selected, changed)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_reference_store_functions_loop5(
    callers: &[String],
    callees: &[String],
    mut selected: Vec<String>,
    mut changed: bool,
) -> Vec<String> {
    loop {
        return if changed {
            {
                let changed_2 = false;
                {
                    let position = 0f64;
                    {
                        let source_ml_source1_s16 =
                            self::source_ml_temporary_plan_formal_reference_store_functions_loop6(
                                callers, callees, selected, changed_2, position,
                            );
                        match source_ml_source1_s16 {
                            self::MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Result::MlTemporaryPlanFormalReferenceStoreFunctionsLoop6Done(selected_2, changed_3) => {
                                {
                                    (selected, changed) = (selected_2, changed_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            selected
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCallableEmissionHasNameLoop9Result {
    MlTemporaryCallableEmissionHasNameLoop9Done,
    MlTemporaryCallableEmissionHasNameLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_callable_emission_has_name_loop9(
    name: &str,
    source_ml_temporary_values8: &[String],
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryCallableEmissionHasNameLoop9Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let previous = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                if previous == name {
                    self::MlTemporaryCallableEmissionHasNameLoop9Result::MlTemporaryCallableEmissionHasNameLoop9Return(true)
                } else {
                    {
                        let source_ml_temporary_index8_2 =
                            source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index8 = source_ml_temporary_index8_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCallableEmissionHasNameLoop9Result::MlTemporaryCallableEmissionHasNameLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadCallableEagerEffectsLoop11Result {
    MlTemporaryReadCallableEagerEffectsLoop11Done,
    MlTemporaryReadCallableEagerEffectsLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_callable_eager_effects_loop11(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    names: &[String],
    mut at: f64,
    mut end: f64,
) -> self::MlTemporaryReadCallableEagerEffectsLoop11Result {
    loop {
        return if at < end {
            if self::accept_callable_effect_field(&(self::read_required_text_fact(labels, at)))
                && self::read_callable_eager_effects(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::read_required_number_fact(targets, at),
                    names,
                )
            {
                self::MlTemporaryReadCallableEagerEffectsLoop11Result::MlTemporaryReadCallableEagerEffectsLoop11Return(true)
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
            self::MlTemporaryReadCallableEagerEffectsLoop11Result::MlTemporaryReadCallableEagerEffectsLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadCallableEagerEffectsJoin10 {
    MlTemporaryReadCallableEagerEffectsJoin10Next,
    MlTemporaryReadCallableEagerEffectsJoin10Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadCallableEffectfulDeclarationsLoop13Result {
    MlTemporaryReadCallableEffectfulDeclarationsLoop13Done(Vec<f64>, Vec<String>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadCallableEffectfulDeclarationsJoin14 {
    MlTemporaryReadCallableEffectfulDeclarationsJoin14Next(Vec<f64>, Vec<String>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_callable_effectful_declarations_loop13(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut selected: Vec<f64>,
    mut names: Vec<String>,
    mut grew: bool,
    mut at: f64,
    mut end: f64,
) -> self::MlTemporaryReadCallableEffectfulDeclarationsLoop13Result {
    loop {
        return if at < end {
            {
                let entry = self::read_required_number_fact(targets, at);
                {
                    let kind = self::callable_emission_text(
                        texts, starts, counts, labels, targets, entry, "k",
                    );
                    {
                        let name = self::callable_emission_text(
                            texts, starts, counts, labels, targets, entry, "fullName",
                        );
                        {
                            let source_ml_source1_s21 = if ((kind == "fn")
                                && !(self::callable_emission_has_name(&(names), &(name))))
                                && self::read_callable_eager_effects(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::callable_emission_field(
                                        starts, counts, labels, targets, entry, "body",
                                    ),
                                    &(names),
                                ) {
                                {
                                    let selected_2 = {
                                        let source_ml_source1_concat_value22 = selected;
                                        {
                                            let source_ml_source1_concat_value23 = vec![entry];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source1_concat_value22,
                                                ),
                                                source_ml_source1_concat_value23,
                                            )
                                        }
                                    };
                                    {
                                        let names_2 = {
                                            let source_ml_source1_concat_value24 = names;
                                            {
                                                let source_ml_source1_concat_value25 =
                                                    vec![name.clone()];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_source1_concat_value24,
                                                    ),
                                                    source_ml_source1_concat_value25,
                                                )
                                            }
                                        };
                                        {
                                            let grew_2 = true;
                                            self::MlTemporaryReadCallableEffectfulDeclarationsJoin14::MlTemporaryReadCallableEffectfulDeclarationsJoin14Next(selected_2, names_2, grew_2)
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryReadCallableEffectfulDeclarationsJoin14::MlTemporaryReadCallableEffectfulDeclarationsJoin14Next(selected, names, grew)
                            };
                            match source_ml_source1_s21 {
                                self::MlTemporaryReadCallableEffectfulDeclarationsJoin14::MlTemporaryReadCallableEffectfulDeclarationsJoin14Next(selected_3, names_3, grew_3) => {
                                    {
                                        let at_2 = at + 1f64;
                                        {
                                            (selected, names, grew, at) = (selected_3, names_3, grew_3, at_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadCallableEffectfulDeclarationsLoop13Result::MlTemporaryReadCallableEffectfulDeclarationsLoop13Done(selected, names, grew)
        };
    }
}
