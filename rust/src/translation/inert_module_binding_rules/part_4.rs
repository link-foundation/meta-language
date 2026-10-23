// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlReadEmptyStandardCollectionModuleBindingLoop34Result;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop34(
    terms: &[String],
    fields: &[String],
    mut constructor: f64,
    mut arguments_at: f64,
    mut news: f64,
    ml_values29: &[f64],
    mut ml_index29: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop34Result {
    loop {
        return if ml_index29 < (self::ml::Big::from_u128(ml_values29.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values29, ml_index29.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "constructor") && (term == "identifier"))
                            && (constructor < 0f64)
                        {
                            {
                                let constructor_2 = child;
                                {
                                    let ml_index29_2 = ml_index29.add(&self::ml::Big::from_i128(1));
                                    {
                                        (constructor, ml_index29) = (constructor_2, ml_index29_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "arguments") && (term == "arguments"))
                                && (arguments_at < 0f64)
                            {
                                {
                                    let arguments_at_2 = child;
                                    {
                                        let ml_index29_3 =
                                            ml_index29.add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, ml_index29) =
                                                (arguments_at_2, ml_index29_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if term == "new" {
                                    {
                                        let news_2 = news + 1f64;
                                        {
                                            let ml_index29_4 =
                                                ml_index29.add(&self::ml::Big::from_i128(1));
                                            {
                                                (news, ml_index29) = (news_2, ml_index29_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadEmptyStandardCollectionModuleBindingLoop34Result::MlReadEmptyStandardCollectionModuleBindingLoop34Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop34Result::MlReadEmptyStandardCollectionModuleBindingLoop34Done(constructor, arguments_at, news)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop33Result {
    MlReadEmptyStandardCollectionModuleBindingLoop33Done(f64, f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop33Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop33(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    ml_values28: &[f64],
    mut ml_index28: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop33Result {
    loop {
        return if ml_index28 < (self::ml::Big::from_u128(ml_values28.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values28, ml_index28.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let ml_index28_2 = ml_index28.add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, ml_index28) = (name_2, ml_index28_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "value") && (term == "new_expression")) && (value < 0f64)
                            {
                                {
                                    let value_2 = child;
                                    {
                                        let ml_index28_3 =
                                            ml_index28.add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, ml_index28) = (value_2, ml_index28_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if term == "=" {
                                    {
                                        let equals_2 = equals + 1f64;
                                        {
                                            let ml_index28_4 =
                                                ml_index28.add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, ml_index28) = (equals_2, ml_index28_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadEmptyStandardCollectionModuleBindingLoop33Result::MlReadEmptyStandardCollectionModuleBindingLoop33Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop33Result::MlReadEmptyStandardCollectionModuleBindingLoop33Done(name, value, equals)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop32Result {
    MlReadEmptyStandardCollectionModuleBindingLoop32Done(f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop32Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop32(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    ml_values27: &[f64],
    mut ml_index27: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop32Result {
    loop {
        return if ml_index27 < (self::ml::Big::from_u128(ml_values27.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values27, ml_index27.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let ml_index27_2 = ml_index27.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, ml_index27) = (constants_2, ml_index27_2);
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
                                    let ml_index27_3 = ml_index27.add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, ml_index27) = (binding_2, ml_index27_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index27_4 = ml_index27.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index27 = ml_index27_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadEmptyStandardCollectionModuleBindingLoop32Result::MlReadEmptyStandardCollectionModuleBindingLoop32Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop32Result::MlReadEmptyStandardCollectionModuleBindingLoop32Done(binding, constants)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingJoin31 {
    MlReadEmptyStandardCollectionModuleBindingJoin31Next(f64),
    MlReadEmptyStandardCollectionModuleBindingJoin31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop36Result {
    MlReadEmptyStandardCollectionModuleBindingLoop36Done(f64, f64),
    MlReadEmptyStandardCollectionModuleBindingLoop36Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop36(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    ml_values26: &[f64],
    mut ml_index26: self::ml::Big,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop36Result {
    loop {
        return if ml_index26 < (self::ml::Big::from_u128(ml_values26.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values26, ml_index26.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let ml_index26_2 = ml_index26.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, ml_index26) = (exports_2, ml_index26_2);
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
                                    let ml_index26_3 = ml_index26.add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, ml_index26) = (selected_2, ml_index26_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index26_4 = ml_index26.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index26 = ml_index26_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadEmptyStandardCollectionModuleBindingLoop36Result::MlReadEmptyStandardCollectionModuleBindingLoop36Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadEmptyStandardCollectionModuleBindingLoop36Result::MlReadEmptyStandardCollectionModuleBindingLoop36Done(selected, exports)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmptyStandardCollectionModuleBindingLoop30Result {
    MlReadEmptyStandardCollectionModuleBindingLoop30Done,
    MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_empty_standard_collection_module_binding_loop30(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlReadEmptyStandardCollectionModuleBindingLoop30Result {
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
                            self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec::<f64>::new())
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
                            self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Return(Vec::<f64>::new())
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
            self::MlReadEmptyStandardCollectionModuleBindingLoop30Result::MlReadEmptyStandardCollectionModuleBindingLoop30Done
        };
    }
}
