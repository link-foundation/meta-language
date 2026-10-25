// Generated from complete js/src/translation/intrinsic-initializer-rules.js. Do not edit by hand.
use super::MlTemporaryReadIntrinsicModuleInitializerLoop38Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop38(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    source_ml_temporary_values31: &[f64],
    mut source_ml_temporary_index31: self::ml::Big,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop38Result {
    loop {
        return if source_ml_temporary_index31
            < (self::ml::Big::from_u128(source_ml_temporary_values31.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values31,
                    source_ml_temporary_index31.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let source_ml_temporary_index31_2 =
                                    source_ml_temporary_index31.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, source_ml_temporary_index31) =
                                        (constants_2, source_ml_temporary_index31_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if (term == "variable_declarator") && (binding < 0f64) {
                            {
                                let binding_2 = child;
                                {
                                    let source_ml_temporary_index31_3 = source_ml_temporary_index31
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, source_ml_temporary_index31) =
                                            (binding_2, source_ml_temporary_index31_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let source_ml_temporary_index31_4 = source_ml_temporary_index31
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index31 = source_ml_temporary_index31_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlTemporaryReadIntrinsicModuleInitializerLoop38Result::MlTemporaryReadIntrinsicModuleInitializerLoop38Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop38Result::MlTemporaryReadIntrinsicModuleInitializerLoop38Done(binding, constants)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerJoin37 {
    MlTemporaryReadIntrinsicModuleInitializerJoin37Next(f64),
    MlTemporaryReadIntrinsicModuleInitializerJoin37Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop44Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop44Done(f64, f64),
    MlTemporaryReadIntrinsicModuleInitializerLoop44Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop44(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    source_ml_temporary_values30: &[f64],
    mut source_ml_temporary_index30: self::ml::Big,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop44Result {
    loop {
        return if source_ml_temporary_index30
            < (self::ml::Big::from_u128(source_ml_temporary_values30.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_temporary_values30,
                    source_ml_temporary_index30.to_index(),
                );
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let source_ml_temporary_index30_2 =
                                    source_ml_temporary_index30.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, source_ml_temporary_index30) =
                                        (exports_2, source_ml_temporary_index30_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if (term == "lexical_declaration") && (selected < 0f64) {
                            {
                                let selected_2 = child;
                                {
                                    let source_ml_temporary_index30_3 = source_ml_temporary_index30
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, source_ml_temporary_index30) =
                                            (selected_2, source_ml_temporary_index30_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let source_ml_temporary_index30_4 = source_ml_temporary_index30
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index30 = source_ml_temporary_index30_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlTemporaryReadIntrinsicModuleInitializerLoop44Result::MlTemporaryReadIntrinsicModuleInitializerLoop44Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop44Result::MlTemporaryReadIntrinsicModuleInitializerLoop44Done(selected, exports)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadIntrinsicModuleInitializerLoop36Result {
    MlTemporaryReadIntrinsicModuleInitializerLoop36Done,
    MlTemporaryReadIntrinsicModuleInitializerLoop36Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_intrinsic_module_initializer_loop36(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result {
    loop {
        return if at < count {
            {
                let parent = self::read_required_number_fact(parents, at);
                {
                    let start = self::read_required_number_fact(starts, at);
                    {
                        let end = self::read_required_number_fact(ends, at);
                        if ((((!(self::ml_math::is_integer(start)) || (start < 0f64))
                            || !(self::ml_math::is_integer(end)))
                            || (end < start))
                            || (end > source_bytes))
                            || ((self::read_required_number_fact(errors, at)) != 0f64)
                        {
                            self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result::MlTemporaryReadIntrinsicModuleInitializerLoop36Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result::MlTemporaryReadIntrinsicModuleInitializerLoop36Return(Vec::<f64>::new())
                            } else {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                        } else if (((!(self::ml_math::is_integer(parent)) || (parent < 0f64))
                            || (parent >= at))
                            || (start < (self::read_required_number_fact(starts, parent))))
                            || (end > (self::read_required_number_fact(ends, parent)))
                        {
                            self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result::MlTemporaryReadIntrinsicModuleInitializerLoop36Return(Vec::<f64>::new())
                        } else {
                            {
                                let at_3 = at + 1f64;
                                {
                                    at = at_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadIntrinsicModuleInitializerLoop36Result::MlTemporaryReadIntrinsicModuleInitializerLoop36Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_intrinsic_module_initializer_scope_loop46(
    mut translated: Vec<String>,
    source_ml_temporary_values45: &[String],
    mut source_ml_temporary_index45: self::ml::Big,
) -> Vec<String> {
    loop {
        if source_ml_temporary_index45
            < (self::ml::Big::from_u128(source_ml_temporary_values45.len() as u128))
        {
            {
                let form = self::ml_array::at(
                    source_ml_temporary_values45,
                    source_ml_temporary_index45.to_index(),
                );
                {
                    let translated_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![if form == "intrinsic value binding" {
                                String::from("owned inert value binding")
                            } else {
                                form.clone()
                            }],
                            translated,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_temporary_index45_2 =
                            source_ml_temporary_index45.add(&self::ml::Big::from_i128(1));
                        {
                            (translated, source_ml_temporary_index45) =
                                (translated_2, source_ml_temporary_index45_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { translated };
    }
}
