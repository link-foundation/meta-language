// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlTemporaryReadCallableEffectfulDeclarationsLoop13Result;
use super::accept_callable_effect_field;
use super::callable_emission_obligations;
use super::callable_emission_text;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_read_callable_effectful_declarations_loop13;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_callable_effectful_declarations_loop12(
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
                            let source_ml_source1_s26 = self::source_ml_temporary_read_callable_effectful_declarations_loop13(forms, texts, starts, counts, labels, targets, selected, names, grew_2, at, end);
                            match source_ml_source1_s26 {
                                self::MlTemporaryReadCallableEffectfulDeclarationsLoop13Result::MlTemporaryReadCallableEffectfulDeclarationsLoop13Done(selected_2, names_2, grew_3) => {
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
pub fn source_ml_temporary_callable_emission_obligations_loop15(
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
                            let source_ml_source1_concat_value27 = selected;
                            {
                                let source_ml_source1_concat_value28 =
                                    self::callable_emission_obligations(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        self::read_required_number_fact(targets, at),
                                        names,
                                    );
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value27,
                                    ),
                                    source_ml_source1_concat_value28,
                                )
                            }
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
pub fn source_ml_temporary_plan_callable_output_obligations_loop17(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut names: Vec<String>,
    source_ml_temporary_values16: &[f64],
    mut source_ml_temporary_index16: self::ml::Big,
) -> Vec<String> {
    loop {
        if source_ml_temporary_index16
            < (self::ml::Big::from_u128(source_ml_temporary_values16.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    source_ml_temporary_values16,
                    source_ml_temporary_index16.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value29 = names;
                        {
                            let source_ml_source1_concat_value30 =
                                vec![self::callable_emission_text(
                                    texts, starts, counts, labels, targets, entry, "fullName",
                                )];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<String>::new(),
                                    source_ml_source1_concat_value29,
                                ),
                                source_ml_source1_concat_value30,
                            )
                        }
                    };
                    {
                        let source_ml_temporary_index16_2 =
                            source_ml_temporary_index16.add(&self::ml::Big::from_i128(1));
                        {
                            (names, source_ml_temporary_index16) =
                                (names_2, source_ml_temporary_index16_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { names };
    }
}
