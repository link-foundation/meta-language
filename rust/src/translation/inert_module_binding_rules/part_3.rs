// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlReadInertModuleBindingLoop20Result;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop20(
    terms: &[String],
    fields: &[String],
    mut callee: f64,
    mut arguments_at: f64,
    ml_values10: &[f64],
    mut ml_index10: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop20Result {
    loop {
        return if ml_index10 < (self::ml::Big::from_u128(ml_values10.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values10, ml_index10.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let part = self::read_required_text_fact(terms, child);
                        if ((field == "function") && (part == "member_expression"))
                            && (callee < 0f64)
                        {
                            {
                                let callee_2 = child;
                                {
                                    let ml_index10_2 = ml_index10.add(&self::ml::Big::from_i128(1));
                                    {
                                        (callee, ml_index10) = (callee_2, ml_index10_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((field == "arguments") && (part == "arguments"))
                                && (arguments_at < 0f64)
                            {
                                {
                                    let arguments_at_2 = child;
                                    {
                                        let ml_index10_3 =
                                            ml_index10.add(&self::ml::Big::from_i128(1));
                                        {
                                            (arguments_at, ml_index10) =
                                                (arguments_at_2, ml_index10_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                self::MlReadInertModuleBindingLoop20Result::MlReadInertModuleBindingLoop20Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop20Result::MlReadInertModuleBindingLoop20Done(
                callee,
                arguments_at,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop14Result {
    MlReadInertModuleBindingLoop14Done(f64, f64, f64),
    MlReadInertModuleBindingLoop14Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop14(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    ml_values6: &[f64],
    mut ml_index6: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop14Result {
    loop {
        return if ml_index6 < (self::ml::Big::from_u128(ml_values6.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values6, ml_index6.to_index());
                {
                    let field = self::read_required_text_fact(fields, child);
                    {
                        let term = self::read_required_text_fact(terms, child);
                        if ((field == "name") && (term == "identifier")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let ml_index6_2 = ml_index6.add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, ml_index6) = (name_2, ml_index6_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (field == "value") && (value < 0f64) {
                                {
                                    let value_2 = child;
                                    {
                                        let ml_index6_3 =
                                            ml_index6.add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, ml_index6) = (value_2, ml_index6_3);
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
                                            let ml_index6_4 =
                                                ml_index6.add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, ml_index6) = (equals_2, ml_index6_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop14Result::MlReadInertModuleBindingLoop14Done(
                name, value, equals,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop13Result {
    MlReadInertModuleBindingLoop13Done(f64, f64),
    MlReadInertModuleBindingLoop13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop13(
    terms: &[String],
    mut binding: f64,
    mut constants: f64,
    ml_values5: &[f64],
    mut ml_index5: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop13Result {
    loop {
        return if ml_index5 < (self::ml::Big::from_u128(ml_values5.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values5, ml_index5.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constants_2 = constants + 1f64;
                            {
                                let ml_index5_2 = ml_index5.add(&self::ml::Big::from_i128(1));
                                {
                                    (constants, ml_index5) = (constants_2, ml_index5_2);
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
                                    let ml_index5_3 = ml_index5.add(&self::ml::Big::from_i128(1));
                                    {
                                        (binding, ml_index5) = (binding_2, ml_index5_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index5_4 = ml_index5.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index5 = ml_index5_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadInertModuleBindingLoop13Result::MlReadInertModuleBindingLoop13Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop13Result::MlReadInertModuleBindingLoop13Done(
                binding, constants,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingJoin12 {
    MlReadInertModuleBindingJoin12Next(f64),
    MlReadInertModuleBindingJoin12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop21Result {
    MlReadInertModuleBindingLoop21Done(f64, f64),
    MlReadInertModuleBindingLoop21Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop21(
    terms: &[String],
    mut selected: f64,
    mut exports: f64,
    ml_values4: &[f64],
    mut ml_index4: self::ml::Big,
) -> self::MlReadInertModuleBindingLoop21Result {
    loop {
        return if ml_index4 < (self::ml::Big::from_u128(ml_values4.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values4, ml_index4.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "export" {
                        {
                            let exports_2 = exports + 1f64;
                            {
                                let ml_index4_2 = ml_index4.add(&self::ml::Big::from_i128(1));
                                {
                                    (exports, ml_index4) = (exports_2, ml_index4_2);
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
                                    let ml_index4_3 = ml_index4.add(&self::ml::Big::from_i128(1));
                                    {
                                        (selected, ml_index4) = (selected_2, ml_index4_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if term == ";" {
                                {
                                    let ml_index4_4 = ml_index4.add(&self::ml::Big::from_i128(1));
                                    {
                                        ml_index4 = ml_index4_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlReadInertModuleBindingLoop21Result::MlReadInertModuleBindingLoop21Return(Vec::<f64>::new())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadInertModuleBindingLoop21Result::MlReadInertModuleBindingLoop21Done(
                selected, exports,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadInertModuleBindingLoop11Result {
    MlReadInertModuleBindingLoop11Done,
    MlReadInertModuleBindingLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_inert_module_binding_loop11(
    terms: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    errors: &[f64],
    mut source_bytes: f64,
    mut at: f64,
) -> self::MlReadInertModuleBindingLoop11Result {
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
                            self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(Vec::<f64>::new())
                        } else if at == 0f64 {
                            if ((parent != -1f64)
                                || ((self::read_required_text_fact(terms, at)) != "program"))
                                || (end != source_bytes)
                            {
                                self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(Vec::<f64>::new())
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
                            self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Return(Vec::<f64>::new())
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
            self::MlReadInertModuleBindingLoop11Result::MlReadInertModuleBindingLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInertModuleBindingScopeLoop23Result {
    MlAcceptInertModuleBindingScopeLoop23Done,
    MlAcceptInertModuleBindingScopeLoop23Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptInertModuleBindingScopeLoop24Result {
    MlAcceptInertModuleBindingScopeLoop24Done,
    MlAcceptInertModuleBindingScopeLoop24Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inert_module_binding_scope_loop24(
    mut owner: f64,
    ml_values22: &[String],
    mut ml_index22: self::ml::Big,
) -> self::MlAcceptInertModuleBindingScopeLoop24Result {
    loop {
        return if ml_index22 < (self::ml::Big::from_u128(ml_values22.len() as u128)) {
            {
                let binding = self::ml_array::at(ml_values22, ml_index22.to_index());
                if (((owner == 1f64) && (binding == "Map"))
                    || ((owner == 2f64) && (binding == "Set")))
                    || ((owner == 3f64) && (binding == "Object"))
                {
                    self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Return(false)
                } else {
                    {
                        let ml_index22_2 = ml_index22.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index22 = ml_index22_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_inert_module_binding_scope_loop23(
    forms: &[String],
    owners: &[f64],
    bindings: &[String],
    mut at: f64,
) -> self::MlAcceptInertModuleBindingScopeLoop23Result {
    loop {
        return if at < (forms.len() as f64) {
            {
                let form = self::read_required_text_fact(forms, at);
                {
                    let owner = self::read_required_number_fact(owners, at);
                    if form == "owned inert value binding" {
                        if (((0f64..=3f64).contains(&owner)) && ((owner % 1f64) == 0f64)) {
                            {
                                let ml_values22 = bindings.to_vec();
                                {
                                    let ml_index22 = self::ml::Big::from_i128(0);
                                    {
                                        let ml_s18 =
                                            self::ml_accept_inert_module_binding_scope_loop24(
                                                owner,
                                                &(ml_values22),
                                                ml_index22,
                                            );
                                        match ml_s18 {
                                            self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Done => {
                                                {
                                                    let at_2 = at + 1f64;
                                                    {
                                                        at = at_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlAcceptInertModuleBindingScopeLoop24Result::MlAcceptInertModuleBindingScopeLoop24Return(ml_result24) => {
                                                self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(ml_result24)
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(false)
                        }
                    } else if (owner != -1f64)
                        || (((((((form != "comment") && (form != "named function declaration"))
                            && (form != "named import"))
                            && (form != "primitive literal binding"))
                            && (form != "empty class declaration"))
                            && (form != "deferred named class declaration"))
                            && (form != "deferred function value binding"))
                    {
                        self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Return(false)
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
        } else {
            self::MlAcceptInertModuleBindingScopeLoop23Result::MlAcceptInertModuleBindingScopeLoop23Done
        };
    }
}
