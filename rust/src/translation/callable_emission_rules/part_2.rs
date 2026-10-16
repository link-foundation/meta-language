// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::accept_callable_effect_field;
use super::callable_emission_field;
use super::callable_emission_has_name;
use super::callable_emission_text;
use super::ml;
use super::ml_array;
use super::read_callable_eager_effects;
use super::read_required_number_fact;
use super::read_required_text_fact;

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
                                                self::ml_callable_emission_obligations_loop12(
                                                    forms, texts, starts, counts, labels, targets,
                                                    names, selected_9, at, end,
                                                )
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
            let ml_values13 = declarations.to_vec();
            {
                let ml_index13 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::ml_plan_callable_output_obligations_loop14(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        names,
                        &(ml_values13),
                        ml_index13,
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
pub enum MlCallableEmissionFieldLoop1Result {
    MlCallableEmissionFieldLoop1Done,
    MlCallableEmissionFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_callable_emission_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut at: f64,
    mut end: f64,
) -> self::MlCallableEmissionFieldLoop1Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlCallableEmissionFieldLoop1Result::MlCallableEmissionFieldLoop1Return(
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
            self::MlCallableEmissionFieldLoop1Result::MlCallableEmissionFieldLoop1Done
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
pub fn ml_plan_callable_capture_bindings_loop4(
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
pub fn ml_plan_callable_capture_bindings_loop3(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut selected: Vec<f64>,
    ml_values2: &[f64],
    mut ml_index2: self::ml::Big,
) -> Vec<f64> {
    loop {
        if ml_index2 < (self::ml::Big::from_u128(ml_values2.len() as u128)) {
            {
                let reference = self::ml_array::at(ml_values2, ml_index2.to_index());
                {
                    let name = self::callable_emission_text(
                        texts, starts, counts, labels, targets, reference, "name",
                    );
                    {
                        let position = -1f64;
                        {
                            let at = 0f64;
                            {
                                let position_2 = self::ml_plan_callable_capture_bindings_loop4(
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
                                            let (ml_append_right, ml_append_left) =
                                                (vec![reference], selected);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let ml_index2_2 =
                                                ml_index2.add(&self::ml::Big::from_i128(1));
                                            {
                                                (selected, ml_index2) = (selected_2, ml_index2_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let selected_3 = self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::slice(&selected, 0f64, position_2),
                                                vec![reference],
                                            ),
                                            self::ml_array::slice(
                                                &selected,
                                                (position_2 + 1f64),
                                                f64::INFINITY,
                                            ),
                                        );
                                        {
                                            let ml_index2_3 =
                                                ml_index2.add(&self::ml::Big::from_i128(1));
                                            {
                                                (selected, ml_index2) = (selected_3, ml_index2_3);
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
pub enum MlCallableEmissionHasNameLoop6Result {
    MlCallableEmissionHasNameLoop6Done,
    MlCallableEmissionHasNameLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_callable_emission_has_name_loop6(
    name: &str,
    ml_values5: &[String],
    mut ml_index5: self::ml::Big,
) -> self::MlCallableEmissionHasNameLoop6Result {
    loop {
        return if ml_index5 < (self::ml::Big::from_u128(ml_values5.len() as u128)) {
            {
                let previous = self::ml_array::at(ml_values5, ml_index5.to_index());
                if previous == name {
                    self::MlCallableEmissionHasNameLoop6Result::MlCallableEmissionHasNameLoop6Return(
                        true,
                    )
                } else {
                    {
                        let ml_index5_2 = ml_index5.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index5 = ml_index5_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlCallableEmissionHasNameLoop6Result::MlCallableEmissionHasNameLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadCallableEagerEffectsLoop8Result {
    MlReadCallableEagerEffectsLoop8Done,
    MlReadCallableEagerEffectsLoop8Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_eager_effects_loop8(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    names: &[String],
    mut at: f64,
    mut end: f64,
) -> self::MlReadCallableEagerEffectsLoop8Result {
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
                self::MlReadCallableEagerEffectsLoop8Result::MlReadCallableEagerEffectsLoop8Return(
                    true,
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
            self::MlReadCallableEagerEffectsLoop8Result::MlReadCallableEagerEffectsLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadCallableEagerEffectsJoin7 {
    MlReadCallableEagerEffectsJoin7Next,
    MlReadCallableEagerEffectsJoin7Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadCallableEffectfulDeclarationsLoop10Result {
    MlReadCallableEffectfulDeclarationsLoop10Done(Vec<f64>, Vec<String>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadCallableEffectfulDeclarationsJoin11 {
    MlReadCallableEffectfulDeclarationsJoin11Next(Vec<f64>, Vec<String>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_callable_effectful_declarations_loop10(
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
) -> self::MlReadCallableEffectfulDeclarationsLoop10Result {
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
                            let ml_s5 = if ((kind == "fn")
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
                                        let (ml_append_right, ml_append_left) =
                                            (vec![entry], selected);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let names_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![name.clone()], names);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let grew_2 = true;
                                            self::MlReadCallableEffectfulDeclarationsJoin11::MlReadCallableEffectfulDeclarationsJoin11Next(selected_2, names_2, grew_2)
                                        }
                                    }
                                }
                            } else {
                                self::MlReadCallableEffectfulDeclarationsJoin11::MlReadCallableEffectfulDeclarationsJoin11Next(selected, names, grew)
                            };
                            match ml_s5 {
                                self::MlReadCallableEffectfulDeclarationsJoin11::MlReadCallableEffectfulDeclarationsJoin11Next(selected_3, names_3, grew_3) => {
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
            self::MlReadCallableEffectfulDeclarationsLoop10Result::MlReadCallableEffectfulDeclarationsLoop10Done(selected, names, grew)
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
pub fn ml_read_callable_effectful_declarations_loop9(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut declarations: f64,
    mut selected: Vec<f64>,
    mut names: Vec<String>,
    mut grew: bool,
) -> Vec<f64> {
    loop {
        return if grew {
            {
                let grew_2 = false;
                {
                    let at = self::read_required_number_fact(starts, declarations);
                    {
                        let end = at + self::read_required_number_fact(counts, declarations);
                        {
                            let ml_s6 = self::ml_read_callable_effectful_declarations_loop10(
                                forms, texts, starts, counts, labels, targets, selected, names,
                                grew_2, at, end,
                            );
                            match ml_s6 {
                                self::MlReadCallableEffectfulDeclarationsLoop10Result::MlReadCallableEffectfulDeclarationsLoop10Done(selected_2, names_2, grew_3) => {
                                    {
                                        (selected, names, grew) = (selected_2, names_2, grew_3);
                                        continue;
                                    }
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_callable_emission_obligations_loop12(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    names: &[String],
    mut selected: Vec<f64>,
    mut at: f64,
    mut end: f64,
) -> Vec<f64> {
    loop {
        if at < end {
            {
                let selected_3 = if self::accept_callable_effect_field(
                    &(self::read_required_text_fact(labels, at)),
                ) {
                    {
                        {
                            let (ml_append_right, ml_append_left) = (
                                self::callable_emission_obligations(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::read_required_number_fact(targets, at),
                                    names,
                                ),
                                selected,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        }
                    }
                } else {
                    selected
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (selected, at) = (selected_3, at_2);
                        continue;
                    }
                }
            }
        }
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_callable_output_obligations_loop14(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut names: Vec<String>,
    ml_values13: &[f64],
    mut ml_index13: self::ml::Big,
) -> Vec<String> {
    loop {
        if ml_index13 < (self::ml::Big::from_u128(ml_values13.len() as u128)) {
            {
                let entry = self::ml_array::at(ml_values13, ml_index13.to_index());
                {
                    let names_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::callable_emission_text(
                                texts, starts, counts, labels, targets, entry, "fullName",
                            )],
                            names,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let ml_index13_2 = ml_index13.add(&self::ml::Big::from_i128(1));
                        {
                            (names, ml_index13) = (names_2, ml_index13_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}
