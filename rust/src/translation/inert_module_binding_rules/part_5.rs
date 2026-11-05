// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNumericModuleBindingJoin44 {
    MlReadNumericModuleBindingJoin44Next(f64),
    MlReadNumericModuleBindingJoin44Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNumericModuleBindingLoop48Result {
    MlReadNumericModuleBindingLoop48Done(f64, f64),
    MlReadNumericModuleBindingLoop48Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_numeric_module_binding_loop48(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    ml_values40: &[f64],
    mut ml_index40: self::ml::Big,
) -> self::MlReadNumericModuleBindingLoop48Result {
    loop {
        return if ml_index40 < (self::ml::Big::from_u128(ml_values40.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values40, ml_index40.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let ml_index40_2 = ml_index40.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, ml_index40) = (exports_2, ml_index40_2);
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
                                    let ml_index40_3 = ml_index40.add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, ml_index40) = (selected_2, ml_index40_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index40_4 = ml_index40.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index40 = ml_index40_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadNumericModuleBindingLoop48Result::MlReadNumericModuleBindingLoop48Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadNumericModuleBindingLoop48Result::MlReadNumericModuleBindingLoop48Done(
                selected, exports,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNumericModuleBindingLoop43Result {
    MlReadNumericModuleBindingLoop43Done,
    MlReadNumericModuleBindingLoop43Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_numeric_module_binding_loop43(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlReadNumericModuleBindingLoop43Result {
    loop {
        return if at < (terms.len() as f64) {
            {
                let parent = self::read_required_number_fact(parents, at);
                {
                    let start = self::read_required_number_fact(starts, at);
                    {
                        let end = self::read_required_number_fact(ends, at);
                        if !(((((start >= 0f64) && ((start % 1f64) == 0f64)) && (end >= start))
                            && ((end % 1f64) == 0f64))
                            && (end <= source_bytes))
                            || ((self::read_required_number_fact(errors, at)) != 0f64)
                        {
                            self::MlReadNumericModuleBindingLoop43Result::MlReadNumericModuleBindingLoop43Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlReadNumericModuleBindingLoop43Result::MlReadNumericModuleBindingLoop43Return(Vec::<f64>::new())
                            } else {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                        } else if (!(((parent >= 0f64) && ((parent % 1f64) == 0f64))
                            && (parent < at))
                            || (start < (self::read_required_number_fact(starts, parent))))
                            || (end > (self::read_required_number_fact(ends, parent)))
                        {
                            self::MlReadNumericModuleBindingLoop43Result::MlReadNumericModuleBindingLoop43Return(Vec::<f64>::new())
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
            self::MlReadNumericModuleBindingLoop43Result::MlReadNumericModuleBindingLoop43Done
        };
    }
}
