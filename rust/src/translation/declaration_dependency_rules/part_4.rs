// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result;
use super::accept_deferred_declaration_parameters;
use super::contains_declaration_dependency;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop27(
    root_owners: &[f64],
    ml_values25: &[f64],
    mut ml_index25: self::ml::Big,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result {
    loop {
        return if ml_index25 < (self::ml::Big::from_u128(ml_values25.len() as u128)) {
            {
                let owner = self::ml_array::at(ml_values25, ml_index25.to_index());
                if (self::contains_declaration_dependency(root_owners, owner)) {
                    {
                        let ml_index25_2 = ml_index25.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index25 = ml_index25_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Return(false)
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result {
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Done(Vec<f64>),
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop26(
    owners: &[f64],
    roles: &[f64],
    mut root_owners: Vec<f64>,
    mut declaration: f64,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result {
    loop {
        return if declaration < (roles.len() as f64) {
            {
                let owner = self::read_required_number_fact(owners, declaration);
                {
                    let role = self::read_required_number_fact(roles, declaration);
                    if !(((owner >= 0f64) && ((owner % 1f64) == 0f64))
                        && (owner < (roles.len() as f64)))
                        || (((role != 0f64) && (role != 1f64)) && (role != 2f64))
                    {
                        self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Return(false)
                    } else if role == 0f64 {
                        if self::contains_declaration_dependency(&(root_owners), owner) {
                            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Return(false)
                        } else {
                            {
                                let root_owners_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![owner], root_owners);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let declaration_2 = declaration + 1f64;
                                    {
                                        (root_owners, declaration) = (root_owners_2, declaration_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let declaration_3 = declaration + 1f64;
                            {
                                declaration = declaration_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop26Done(root_owners)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result {
    MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Done,
    MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_correspondence_loop34(
    whole: &[f64],
    member: &[f64],
    mut at: f64,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result {
    loop {
        return if at < (whole.len() as f64) {
            if (self::read_required_number_fact(whole, at))
                == (self::read_required_number_fact(member, at))
            {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                self::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Return(false)
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result {
    MlAcceptRecursiveDeclarationOwnedEmissionLoop37Done(f64),
    MlAcceptRecursiveDeclarationOwnedEmissionLoop37Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptRecursiveDeclarationOwnedEmissionLoop38Result {
    MlAcceptRecursiveDeclarationOwnedEmissionLoop38Done(Vec<f64>),
    MlAcceptRecursiveDeclarationOwnedEmissionLoop38Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_owned_emission_loop38(
    mappings: &[f64],
    forbidden_targets: &[f64],
    mut targets: Vec<f64>,
    mut declaration: f64,
    mut start: f64,
    mut count: f64,
    mut position: f64,
) -> self::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Result {
    loop {
        return if position < count {
            {
                let at = (start + position) * 3f64;
                {
                    let target = self::read_required_number_fact(mappings, (at + 2f64));
                    if (((((self::read_required_number_fact(mappings, at)) != declaration)
                        || ((self::read_required_number_fact(mappings, (at + 1f64))) != position))
                        || !(((target >= 0f64) && ((target % 1f64) == 0f64))
                            && (target <= 9_007_199_254_740_991f64)))
                        || self::contains_declaration_dependency(&(targets), target))
                        || self::contains_declaration_dependency(forbidden_targets, target)
                    {
                        self::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Return(false)
                    } else {
                        {
                            let targets_2 = {
                                let (ml_append_right, ml_append_left) = (vec![target], targets);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let position_2 = position + 1f64;
                                {
                                    (targets, position) = (targets_2, position_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Done(targets)
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
pub fn ml_accept_recursive_declaration_owned_emission_loop37(
    roles: &[f64],
    constructor_counts: &[f64],
    blocks: &[f64],
    mappings: &[f64],
    forbidden_targets: &[f64],
    mut declarations: Vec<f64>,
    mut targets: Vec<f64>,
    mut mapping_offset: f64,
    mut block: f64,
) -> self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result {
    loop {
        return if block < (roles.len() as f64) {
            {
                let declaration = self::read_required_number_fact(blocks, (block * 4f64));
                {
                    let start = self::read_required_number_fact(blocks, ((block * 4f64) + 1f64));
                    {
                        let count =
                            self::read_required_number_fact(blocks, ((block * 4f64) + 2f64));
                        {
                            let index =
                                self::read_required_number_fact(blocks, ((block * 4f64) + 3f64));
                            if ((!(((declaration >= 0f64)
                                && (declaration < (roles.len() as f64)))
                                && ((declaration % 1f64) == 0f64))
                                || self::contains_declaration_dependency(
                                    &(declarations),
                                    declaration,
                                ))
                                || (start != mapping_offset))
                                || (index != block)
                            {
                                self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Return(false)
                            } else {
                                {
                                    let declarations_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![declaration], declarations);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let role =
                                            self::read_required_number_fact(roles, declaration);
                                        {
                                            let constructors = self::read_required_number_fact(
                                                constructor_counts,
                                                declaration,
                                            );
                                            if ((((((role != 0f64) && (role != 1f64))
                                                && (role != 2f64))
                                                || !((constructors >= 0f64)
                                                    && ((constructors % 1f64) == 0f64)))
                                                || ((role != 2f64) && (constructors != 0f64)))
                                                || (count != (constructors + 1f64)))
                                                || (count
                                                    > (((mappings.len() as f64) / 3f64) - start))
                                            {
                                                self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Return(false)
                                            } else {
                                                {
                                                    let position = 0f64;
                                                    {
                                                        let ml_s4400 = self::ml_accept_recursive_declaration_owned_emission_loop38(mappings, forbidden_targets, targets, declaration, start, count, position);
                                                        match ml_s4400 {
                                                            self::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Done(targets_2) => {
                                                                {
                                                                    let mapping_offset_2 = mapping_offset + count;
                                                                    {
                                                                        let block_2 = block + 1f64;
                                                                        {
                                                                            (declarations, targets, mapping_offset, block) = (declarations_2, targets_2, mapping_offset_2, block_2);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop38Return(ml_result38) => {
                                                                self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Return(ml_result38)
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
            }
        } else {
            self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Done(mapping_offset)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result {
    MlAcceptRecursiveDeclarationOwnedEmissionLoop36Done,
    MlAcceptRecursiveDeclarationOwnedEmissionLoop36Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_owned_emission_loop36(
    mut occupied: Vec<f64>,
    ml_values35: &[f64],
    mut ml_index35: self::ml::Big,
) -> self::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result {
    loop {
        return if ml_index35 < (self::ml::Big::from_u128(ml_values35.len() as u128)) {
            {
                let target = self::ml_array::at(ml_values35, ml_index35.to_index());
                if !(((target >= 0f64) && ((target % 1f64) == 0f64))
                    && (target <= 9_007_199_254_740_991f64))
                    || self::contains_declaration_dependency(&(occupied), target)
                {
                    self::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Return(false)
                } else {
                    {
                        let occupied_2 = {
                            let (ml_append_right, ml_append_left) = (vec![target], occupied);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let ml_index35_2 = ml_index35.add(&self::ml::Big::from_i128(1));
                            {
                                (occupied, ml_index35) = (occupied_2, ml_index35_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result {
    MlAcceptRecursiveDeclarationOwnedPublicationLoop40Done,
    MlAcceptRecursiveDeclarationOwnedPublicationLoop40Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_owned_publication_loop40(
    ml_values39: &[f64],
    mut ml_index39: self::ml::Big,
) -> self::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result {
    loop {
        return if ml_index39 < (self::ml::Big::from_u128(ml_values39.len() as u128)) {
            {
                let proof = self::ml_array::at(ml_values39, ml_index39.to_index());
                {
                    let ml_library_inequality_1 = proof != 1f64;
                    if ml_library_inequality_1 {
                        self::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Return(false)
                    } else {
                        {
                            let ml_index39_2 = ml_index39.add(&self::ml::Big::from_i128(1));
                            {
                                ml_index39 = ml_index39_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_deferred_declaration_children_loop41(
    terms: &[String],
    parents: &[f64],
    mut parent: f64,
    mut children: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (terms.len() as f64) {
            {
                let term = self::read_required_text_fact(terms, at);
                if (((self::read_required_number_fact(parents, at)) == parent)
                    && (term != "comment"))
                    && (term != "whitespace")
                {
                    {
                        let children_2 = {
                            let (ml_append_right, ml_append_left) = (vec![at], children);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (children, at) = (children_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
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
        return { children };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptDeferredDeclarationParametersLoop44Result {
    MlAcceptDeferredDeclarationParametersLoop44Done,
    MlAcceptDeferredDeclarationParametersLoop44Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptDeferredDeclarationParametersLoop45Result {
    MlAcceptDeferredDeclarationParametersLoop45Done,
    MlAcceptDeferredDeclarationParametersLoop45Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_deferred_declaration_parameters_loop45(
    text: &str,
    ml_values43: &[String],
    mut ml_index43: self::ml::Big,
) -> self::MlAcceptDeferredDeclarationParametersLoop45Result {
    loop {
        return if ml_index43 < (self::ml::Big::from_u128(ml_values43.len() as u128)) {
            {
                let name = self::ml_array::at(ml_values43, ml_index43.to_index());
                if name == text {
                    self::MlAcceptDeferredDeclarationParametersLoop45Result::MlAcceptDeferredDeclarationParametersLoop45Return(false)
                } else {
                    {
                        let ml_index43_2 = ml_index43.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index43 = ml_index43_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptDeferredDeclarationParametersLoop45Result::MlAcceptDeferredDeclarationParametersLoop45Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_deferred_declaration_parameters_loop44(
    terms: &[String],
    texts: &[String],
    mut names: Vec<String>,
    ml_values42: &[f64],
    mut ml_index42: self::ml::Big,
) -> self::MlAcceptDeferredDeclarationParametersLoop44Result {
    loop {
        return if ml_index42 < (self::ml::Big::from_u128(ml_values42.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values42, ml_index42.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if ((term == "(") || (term == ")")) || (term == ",") {
                        {
                            let ml_index42_2 = ml_index42.add(&self::ml::Big::from_i128(1));
                            {
                                ml_index42 = ml_index42_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let ml_library_inequality_1 = term != "identifier";
                            if ml_library_inequality_1 {
                                self::MlAcceptDeferredDeclarationParametersLoop44Result::MlAcceptDeferredDeclarationParametersLoop44Return(false)
                            } else {
                                {
                                    let text = self::read_required_text_fact(texts, child);
                                    if ((((text.encode_utf16().count() as f64) == 0f64)
                                        || text.contains('\\'))
                                        || (text == "eval"))
                                        || (text == "arguments")
                                    {
                                        self::MlAcceptDeferredDeclarationParametersLoop44Result::MlAcceptDeferredDeclarationParametersLoop44Return(false)
                                    } else {
                                        {
                                            let ml_values43 = names.clone();
                                            {
                                                let ml_index43 = self::ml::Big::from_i128(0);
                                                {
                                                    let ml_s4411 = self::ml_accept_deferred_declaration_parameters_loop45(&(text), &(ml_values43), ml_index43);
                                                    match ml_s4411 {
                                                    self::MlAcceptDeferredDeclarationParametersLoop45Result::MlAcceptDeferredDeclarationParametersLoop45Done => {
                                                        {
                                                            let names_2 = { let (ml_append_right, ml_append_left) = (vec![text.clone()], names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            {
                                                                let ml_index42_3 = ml_index42.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (names, ml_index42) = (names_2, ml_index42_3);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlAcceptDeferredDeclarationParametersLoop45Result::MlAcceptDeferredDeclarationParametersLoop45Return(ml_result45) => {
                                                        self::MlAcceptDeferredDeclarationParametersLoop44Result::MlAcceptDeferredDeclarationParametersLoop44Return(ml_result45)
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
        } else {
            self::MlAcceptDeferredDeclarationParametersLoop44Result::MlAcceptDeferredDeclarationParametersLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingJoin55 {
    MlReadDeclarationDeferredBindingJoin55Next(f64),
    MlReadDeclarationDeferredBindingJoin55Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop58Result {
    MlReadDeclarationDeferredBindingLoop58Done(f64, f64),
    MlReadDeclarationDeferredBindingLoop58Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_declaration_deferred_binding_loop58(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    texts: &[String],
    mut parameters: f64,
    mut body: f64,
    ml_values49: &[f64],
    mut ml_index49: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop58Result {
    loop {
        return if ml_index49 < (self::ml::Big::from_u128(ml_values49.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values49, ml_index49.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let field = self::read_required_text_fact(fields, child);
                        if (field == "parameters") && (term == "formal_parameters") {
                            if (self::accept_deferred_declaration_parameters(
                                terms, parents, texts, child,
                            )) {
                                {
                                    let parameters_2 = parameters + 1f64;
                                    {
                                        let ml_index49_2 =
                                            ml_index49.add(&self::ml::Big::from_i128(1));
                                        {
                                            (parameters, ml_index49) = (parameters_2, ml_index49_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                self::MlReadDeclarationDeferredBindingLoop58Result::MlReadDeclarationDeferredBindingLoop58Return(Vec::<f64>::new())
                            }
                        } else if (field == "parameter") && (term == "identifier") {
                            {
                                let text = self::read_required_text_fact(texts, child);
                                if ((((text.encode_utf16().count() as f64) == 0f64)
                                    || text.contains('\\'))
                                    || (text == "eval"))
                                    || (text == "arguments")
                                {
                                    self::MlReadDeclarationDeferredBindingLoop58Result::MlReadDeclarationDeferredBindingLoop58Return(Vec::<f64>::new())
                                } else {
                                    {
                                        let parameters_3 = parameters + 1f64;
                                        {
                                            let ml_index49_3 =
                                                ml_index49.add(&self::ml::Big::from_i128(1));
                                            {
                                                (parameters, ml_index49) =
                                                    (parameters_3, ml_index49_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if field == "body" {
                                {
                                    let body_2 = body + 1f64;
                                    {
                                        let ml_index49_4 =
                                            ml_index49.add(&self::ml::Big::from_i128(1));
                                        {
                                            (body, ml_index49) = (body_2, ml_index49_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if ((term == "function") || (term == "=>"))
                                    || ((field == "name") && (term == "identifier"))
                                {
                                    {
                                        let ml_index49_5 =
                                            ml_index49.add(&self::ml::Big::from_i128(1));
                                        {
                                            ml_index49 = ml_index49_5;
                                            continue;
                                        }
                                    }
                                }
                                {
                                    self::MlReadDeclarationDeferredBindingLoop58Result::MlReadDeclarationDeferredBindingLoop58Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop58Result::MlReadDeclarationDeferredBindingLoop58Done(parameters, body)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop57Result {
    MlReadDeclarationDeferredBindingLoop57Done(f64, f64, f64),
    MlReadDeclarationDeferredBindingLoop57Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_deferred_binding_loop57(
    terms: &[String],
    fields: &[String],
    mut name: f64,
    mut value: f64,
    mut equals: f64,
    ml_values48: &[f64],
    mut ml_index48: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop57Result {
    loop {
        return if ml_index48 < (self::ml::Big::from_u128(ml_values48.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values48, ml_index48.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    {
                        let field = self::read_required_text_fact(fields, child);
                        if ((term == "identifier") && (field == "name")) && (name < 0f64) {
                            {
                                let name_2 = child;
                                {
                                    let ml_index48_2 = ml_index48.add(&self::ml::Big::from_i128(1));
                                    {
                                        (name, ml_index48) = (name_2, ml_index48_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if (((term == "arrow_function") || (term == "function_expression"))
                                && (field == "value"))
                                && (value < 0f64)
                            {
                                {
                                    let value_2 = child;
                                    {
                                        let ml_index48_3 =
                                            ml_index48.add(&self::ml::Big::from_i128(1));
                                        {
                                            (value, ml_index48) = (value_2, ml_index48_3);
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
                                            let ml_index48_4 =
                                                ml_index48.add(&self::ml::Big::from_i128(1));
                                            {
                                                (equals, ml_index48) = (equals_2, ml_index48_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlReadDeclarationDeferredBindingLoop57Result::MlReadDeclarationDeferredBindingLoop57Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop57Result::MlReadDeclarationDeferredBindingLoop57Done(name, value, equals)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop56Result {
    MlReadDeclarationDeferredBindingLoop56Done(f64, f64),
    MlReadDeclarationDeferredBindingLoop56Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_deferred_binding_loop56(
    terms: &[String],
    mut constant: f64,
    mut binding: f64,
    ml_values47: &[f64],
    mut ml_index47: self::ml::Big,
) -> self::MlReadDeclarationDeferredBindingLoop56Result {
    loop {
        return if ml_index47 < (self::ml::Big::from_u128(ml_values47.len() as u128)) {
            {
                let child = self::ml_array::at(ml_values47, ml_index47.to_index());
                {
                    let term = self::read_required_text_fact(terms, child);
                    if term == "const" {
                        {
                            let constant_2 = constant + 1f64;
                            {
                                let ml_index47_2 = ml_index47.add(&self::ml::Big::from_i128(1));
                                {
                                    (constant, ml_index47) = (constant_2, ml_index47_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if term == "variable_declarator" {
                            if binding >= 0f64 {
                                self::MlReadDeclarationDeferredBindingLoop56Result::MlReadDeclarationDeferredBindingLoop56Return(Vec::<f64>::new())
                            } else {
                                {
                                    let binding_2 = child;
                                    {
                                        let ml_index47_3 =
                                            ml_index47.add(&self::ml::Big::from_i128(1));
                                        {
                                            (binding, ml_index47) = (binding_2, ml_index47_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if term == ";" {
                            {
                                let ml_index47_4 = ml_index47.add(&self::ml::Big::from_i128(1));
                                {
                                    ml_index47 = ml_index47_4;
                                    continue;
                                }
                            }
                        } else {
                            self::MlReadDeclarationDeferredBindingLoop56Result::MlReadDeclarationDeferredBindingLoop56Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDeferredBindingLoop56Result::MlReadDeclarationDeferredBindingLoop56Done(constant, binding)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop60Result {
    MlReadDeclarationDeferredBindingLoop60Done,
    MlReadDeclarationDeferredBindingLoop60Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDeferredBindingLoop61Result {
    MlReadDeclarationDeferredBindingLoop61Done(f64, f64, f64),
    MlReadDeclarationDeferredBindingLoop61Return(Vec<f64>),
}
