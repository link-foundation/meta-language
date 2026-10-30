// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlAcceptDeclarationDeferredModuleScopeLoop64Result;
use super::MlAcceptDeferredDeclarationParametersLoop44Result;
use super::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result;
use super::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result;
use super::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result;
use super::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result;
use super::MlReadDeclarationDeferredBindingJoin54;
use super::MlReadDeclarationDeferredBindingJoin55;
use super::MlReadDeclarationDeferredBindingLoop53Result;
use super::MlReadDeclarationDeferredBindingLoop56Result;
use super::MlReadDeclarationDeferredBindingLoop57Result;
use super::MlReadDeclarationDeferredBindingLoop58Result;
use super::MlReadDeclarationDeferredBindingLoop59Result;
use super::MlReadDeclarationDeferredBindingLoop60Result;
use super::MlReadDeclarationDeferredBindingLoop62Result;
use super::ml;
use super::ml_accept_declaration_deferred_module_scope_loop64;
use super::ml_accept_deferred_declaration_parameters_loop44;
use super::ml_accept_recursive_declaration_auxiliary_correspondence_loop34;
use super::ml_accept_recursive_declaration_owned_emission_loop36;
use super::ml_accept_recursive_declaration_owned_emission_loop37;
use super::ml_accept_recursive_declaration_owned_publication_loop40;
use super::ml_array;
use super::ml_read_declaration_deferred_binding_loop53;
use super::ml_read_declaration_deferred_binding_loop56;
use super::ml_read_declaration_deferred_binding_loop57;
use super::ml_read_declaration_deferred_binding_loop58;
use super::ml_read_declaration_deferred_binding_loop59;
use super::ml_read_declaration_deferred_binding_loop60;
use super::ml_read_declaration_deferred_binding_loop62;
use super::ml_read_deferred_declaration_children_loop41;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_recursive_declaration_auxiliary_correspondence(
    whole: &[f64],
    member: &[f64],
) -> bool {
    if !({
        let ml_value = whole.len() as f64;
        ml_value != 0f64 && !ml_value.is_nan()
    }) || ((whole.len() as f64) != (member.len() as f64))
    {
        false
    } else {
        {
            let at = 0f64;
            {
                let ml_s1122 =
                    self::ml_accept_recursive_declaration_auxiliary_correspondence_loop34(
                        whole, member, at,
                    );
                match ml_s1122 {
                    self::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Done => {
                        true
                    }
                    self::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Result::MlAcceptRecursiveDeclarationAuxiliaryCorrespondenceLoop34Return(ml_result34) => {
                        ml_result34
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_recursive_declaration_owned_emission(
    roles: &[f64],
    constructor_counts: &[f64],
    blocks: &[f64],
    mappings: &[f64],
    forbidden_targets: &[f64],
) -> bool {
    if ((!({
        let ml_value = roles.len() as f64;
        ml_value != 0f64 && !ml_value.is_nan()
    }) || ((roles.len() as f64) != (constructor_counts.len() as f64)))
        || ((blocks.len() as f64) != ((roles.len() as f64) * 4f64)))
        || (((mappings.len() as f64) % 3f64) != 0f64)
    {
        false
    } else {
        {
            let declarations = Vec::<f64>::new();
            {
                let targets = Vec::<f64>::new();
                {
                    let mapping_offset = 0f64;
                    {
                        let occupied = Vec::<f64>::new();
                        {
                            let ml_values35 = forbidden_targets.to_vec();
                            {
                                let ml_index35 = self::ml::Big::from_i128(0);
                                {
                                    let ml_s1133 =
                                        self::ml_accept_recursive_declaration_owned_emission_loop36(
                                            occupied,
                                            &(ml_values35),
                                            ml_index35,
                                        );
                                    match ml_s1133 {
                                        self::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Done => {
                                            {
                                                let block = 0f64;
                                                {
                                                    let ml_s1144 = self::ml_accept_recursive_declaration_owned_emission_loop37(roles, constructor_counts, blocks, mappings, forbidden_targets, declarations, targets, mapping_offset, block);
                                                    match ml_s1144 {
                                                        self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Done(mapping_offset_2) => {
                                                            mapping_offset_2 == ((mappings.len() as f64) / 3f64)
                                                        }
                                                        self::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop37Return(ml_result37) => {
                                                            ml_result37
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Result::MlAcceptRecursiveDeclarationOwnedEmissionLoop36Return(ml_result38) => {
                                            ml_result38
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
pub fn accept_recursive_declaration_owned_publication(
    members: f64,
    declarations: f64,
    signatures: f64,
    completed: f64,
    proofs: &[f64],
) -> bool {
    if (((((((members > 1f64) && ((members % 1f64) == 0f64))
        && (members <= 9_007_199_254_740_991f64))
        && (((declarations > members) && ((declarations % 1f64) == 0f64))
            && (declarations <= 9_007_199_254_740_991f64)))
        && (signatures == members))
        && (completed == members))
        && ((proofs.len() as f64) == 7f64))
    {
        {
            let ml_values39 = proofs.to_vec();
            {
                let ml_index39 = self::ml::Big::from_i128(0);
                {
                    let ml_s1155 = self::ml_accept_recursive_declaration_owned_publication_loop40(
                        &(ml_values39),
                        ml_index39,
                    );
                    match ml_s1155 {
                        self::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Done => {
                            true
                        }
                        self::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Result::MlAcceptRecursiveDeclarationOwnedPublicationLoop40Return(ml_result40) => {
                            ml_result40
                        }
                    }
                }
            }
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_deferred_declaration_children(
    terms: &[String],
    parents: &[f64],
    parent: f64,
) -> Vec<f64> {
    {
        let children = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                self::ml_read_deferred_declaration_children_loop41(
                    terms, parents, parent, children, at,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_deferred_declaration_parameters(
    terms: &[String],
    parents: &[f64],
    texts: &[String],
    owner: f64,
) -> bool {
    {
        let names = Vec::<String>::new();
        {
            let ml_values42 = self::read_deferred_declaration_children(terms, parents, owner);
            {
                let ml_index42 = self::ml::Big::from_i128(0);
                {
                    let ml_s1166 = self::ml_accept_deferred_declaration_parameters_loop44(
                        terms,
                        texts,
                        names,
                        &(ml_values42),
                        ml_index42,
                    );
                    match ml_s1166 {
                        self::MlAcceptDeferredDeclarationParametersLoop44Result::MlAcceptDeferredDeclarationParametersLoop44Done => {
                            true
                        }
                        self::MlAcceptDeferredDeclarationParametersLoop44Result::MlAcceptDeferredDeclarationParametersLoop44Return(ml_result44) => {
                            ml_result44
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
pub fn read_declaration_deferred_binding(
    terms: &[String],
    fields: &[String],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    texts: &[String],
    errors: &[f64],
    source_bytes: f64,
) -> Vec<f64> {
    if ((((((((terms.len() as f64) < 1f64) || ((fields.len() as f64) != (terms.len() as f64)))
        || ((parents.len() as f64) != (terms.len() as f64)))
        || ((starts.len() as f64) != (terms.len() as f64)))
        || ((ends.len() as f64) != (terms.len() as f64)))
        || ((texts.len() as f64) != (terms.len() as f64)))
        || ((errors.len() as f64) != (terms.len() as f64)))
        || !(((source_bytes >= 0f64) && ((source_bytes % 1f64) == 0f64))
            && (source_bytes <= 9_007_199_254_740_991f64))
    {
        Vec::<f64>::new()
    } else {
        {
            let at = 0f64;
            {
                let ml_s1177 = self::ml_read_declaration_deferred_binding_loop53(
                    terms,
                    parents,
                    starts,
                    ends,
                    errors,
                    source_bytes,
                    at,
                );
                match ml_s1177 {
                    self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Done => {
                        {
                            let top = self::read_deferred_declaration_children(terms, parents, 0f64);
                            {
let ml_library_inequality_1 = (top.len() as f64) != 1f64 ;
if ml_library_inequality_1 {
                                Vec::<f64>::new()
                            } else {
                                {
                                    let declaration = self::read_required_number_fact(&(top), 0f64);
                                    {
                                        let ml_s1199 = if (self::read_required_text_fact(terms, declaration)) == "export_statement" {
                                            {
                                                let selected = -1f64;
                                                {
                                                    let exports = 0f64;
                                                    {
                                                        let ml_values46 = self::read_deferred_declaration_children(terms, parents, declaration);
                                                        {
                                                            let ml_index46 = self::ml::Big::from_i128(0);
                                                            {
                                                                let ml_s2200 = self::ml_read_declaration_deferred_binding_loop62(terms, selected, exports, &(ml_values46), ml_index46);
                                                                match ml_s2200 {
                                                                    self::MlReadDeclarationDeferredBindingLoop62Result::MlReadDeclarationDeferredBindingLoop62Done(selected_2, exports_2) => {
                                                                        if (exports_2 != 1f64) || (selected_2 < 0f64) {
                                                                            self::MlReadDeclarationDeferredBindingJoin54::MlReadDeclarationDeferredBindingJoin54Return(Vec::<f64>::new())
                                                                        } else {
                                                                            {
                                                                                let declaration_2 = selected_2;
                                                                                self::MlReadDeclarationDeferredBindingJoin54::MlReadDeclarationDeferredBindingJoin54Next(declaration_2)
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlReadDeclarationDeferredBindingLoop62Result::MlReadDeclarationDeferredBindingLoop62Return(ml_result62) => {
                                                                        self::MlReadDeclarationDeferredBindingJoin54::MlReadDeclarationDeferredBindingJoin54Return(ml_result62)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlReadDeclarationDeferredBindingJoin54::MlReadDeclarationDeferredBindingJoin54Next(declaration)
                                        };
                                        match ml_s1199 {
                                            self::MlReadDeclarationDeferredBindingJoin54::MlReadDeclarationDeferredBindingJoin54Next(declaration_3) => {
                                                {
                                                    let kind = self::read_required_text_fact(terms, declaration_3);
                                                    {
                                                        let name = -1f64;
                                                        {
                                                            let ml_s2266 = if kind == "lexical_declaration" {
                                                                {
                                                                    let constant = 0f64;
                                                                    {
                                                                        let binding = -1f64;
                                                                        {
                                                                            let ml_values47 = self::read_deferred_declaration_children(terms, parents, declaration_3);
                                                                            {
                                                                                let ml_index47 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let ml_s2277 = self::ml_read_declaration_deferred_binding_loop56(terms, constant, binding, &(ml_values47), ml_index47);
                                                                                    match ml_s2277 {
                                                                                        self::MlReadDeclarationDeferredBindingLoop56Result::MlReadDeclarationDeferredBindingLoop56Done(constant_2, binding_2) => {
                                                                                            if (constant_2 != 1f64) || (binding_2 < 0f64) {
                                                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(Vec::<f64>::new())
                                                                                            } else {
                                                                                                {
                                                                                                    let value = -1f64;
                                                                                                    {
                                                                                                        let equals = 0f64;
                                                                                                        {
                                                                                                            let ml_values48 = self::read_deferred_declaration_children(terms, parents, binding_2);
                                                                                                            {
                                                                                                                let ml_index48 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let ml_s2288 = self::ml_read_declaration_deferred_binding_loop57(terms, fields, name, value, equals, &(ml_values48), ml_index48);
                                                                                                                    match ml_s2288 {
                                                                                                                        self::MlReadDeclarationDeferredBindingLoop57Result::MlReadDeclarationDeferredBindingLoop57Done(name_2, value_2, equals_2) => {
                                                                                                                            if ((name_2 < 0f64) || (value_2 < 0f64)) || (equals_2 != 1f64) {
                                                                                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(Vec::<f64>::new())
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let parameters = 0f64;
                                                                                                                                    {
                                                                                                                                        let body = 0f64;
                                                                                                                                        {
                                                                                                                                            let ml_values49 = self::read_deferred_declaration_children(terms, parents, value_2);
                                                                                                                                            {
                                                                                                                                                let ml_index49 = self::ml::Big::from_i128(0);
                                                                                                                                                {
                                                                                                                                                    let ml_s2299 = self::ml_read_declaration_deferred_binding_loop58(terms, fields, parents, texts, parameters, body, &(ml_values49), ml_index49);
                                                                                                                                                    match ml_s2299 {
                                                                                                                                                        self::MlReadDeclarationDeferredBindingLoop58Result::MlReadDeclarationDeferredBindingLoop58Done(parameters_2, body_2) => {
                                                                                                                                                            if (parameters_2 != 1f64) || (body_2 != 1f64) {
                                                                                                                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(Vec::<f64>::new())
                                                                                                                                                            } else {
                                                                                                                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Next(name_2)
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        self::MlReadDeclarationDeferredBindingLoop58Result::MlReadDeclarationDeferredBindingLoop58Return(ml_result58) => {
                                                                                                                                                            self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(ml_result58)
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlReadDeclarationDeferredBindingLoop57Result::MlReadDeclarationDeferredBindingLoop57Return(ml_result58_2) => {
                                                                                                                            self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(ml_result58_2)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlReadDeclarationDeferredBindingLoop56Result::MlReadDeclarationDeferredBindingLoop56Return(ml_result58_3) => {
                                                                                            self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(ml_result58_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else if kind == "class_declaration" {
                                                                    {
                                                                        let body_3 = -1f64;
                                                                        {
                                                                            let classes = 0f64;
                                                                            {
                                                                                let ml_values50 = self::read_deferred_declaration_children(terms, parents, declaration_3);
                                                                                {
                                                                                    let ml_index50 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let ml_s3300 = self::ml_read_declaration_deferred_binding_loop59(terms, fields, name, body_3, classes, &(ml_values50), ml_index50);
                                                                                        match ml_s3300 {
                                                                                            self::MlReadDeclarationDeferredBindingLoop59Result::MlReadDeclarationDeferredBindingLoop59Done(name_3, body_4, classes_2) => {
                                                                                                if ((classes_2 != 1f64) || (name_3 < 0f64)) || (body_4 < 0f64) {
                                                                                                    self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(Vec::<f64>::new())
                                                                                                } else {
                                                                                                    {
                                                                                                        let constructors = 0f64;
                                                                                                        {
                                                                                                            let ml_values51 = self::read_deferred_declaration_children(terms, parents, body_4);
                                                                                                            {
                                                                                                                let ml_index51 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let ml_s3311 = self::ml_read_declaration_deferred_binding_loop60(terms, fields, parents, texts, constructors, &(ml_values51), ml_index51);
                                                                                                                    match ml_s3311 {
                                                                                                                        self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Done => {
                                                                                                                            self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Next(name_3)
                                                                                                                        }
                                                                                                                        self::MlReadDeclarationDeferredBindingLoop60Result::MlReadDeclarationDeferredBindingLoop60Return(ml_result60) => {
                                                                                                                            self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(ml_result60)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlReadDeclarationDeferredBindingLoop59Result::MlReadDeclarationDeferredBindingLoop59Return(ml_result61) => {
                                                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(ml_result61)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(Vec::<f64>::new())
                                                                };
                                                            match ml_s2266 {
                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Next(name_4) => {
                                                                    {
                                                                        let actual_name = self::read_required_text_fact(texts, name_4);
                                                                        if ((((actual_name.encode_utf16().count() as f64) == 0f64) || actual_name.contains('\\')) || (actual_name == "eval")) || (actual_name == "arguments") {
                                                                            Vec::<f64>::new()
                                                                        } else {
                                                                            vec![if kind == "class_declaration" {
                                                                                1f64
                                                                            } else {
                                                                                2f64
                                                                            }, name_4]
                                                                        }
                                                                    }
                                                                }
                                                                self::MlReadDeclarationDeferredBindingJoin55::MlReadDeclarationDeferredBindingJoin55Return(ml_result55) => {
                                                                    ml_result55
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlReadDeclarationDeferredBindingJoin54::MlReadDeclarationDeferredBindingJoin54Return(ml_result61_2) => {
                                                ml_result61_2
                                            }
                                        }
                                    }
                                }
                            }
}
                        }
                    }
                    self::MlReadDeclarationDeferredBindingLoop53Result::MlReadDeclarationDeferredBindingLoop53Return(ml_result62_2) => {
                        ml_result62_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_declaration_deferred_module_scope(forms: &[String]) -> bool {
    {
        let ml_values63 = forms.to_vec();
        {
            let ml_index63 = self::ml::Big::from_i128(0);
            {
                let ml_s3322 = self::ml_accept_declaration_deferred_module_scope_loop64(
                    &(ml_values63),
                    ml_index63,
                );
                match ml_s3322 {
                    self::MlAcceptDeclarationDeferredModuleScopeLoop64Result::MlAcceptDeclarationDeferredModuleScopeLoop64Done => {
                        true
                    }
                    self::MlAcceptDeclarationDeferredModuleScopeLoop64Result::MlAcceptDeclarationDeferredModuleScopeLoop64Return(ml_result64) => {
                        ml_result64
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlContainsDeclarationDependencyLoop2Result {
    MlContainsDeclarationDependencyLoop2Done,
    MlContainsDeclarationDependencyLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_contains_declaration_dependency_loop2(
    mut value: f64,
    ml_values1: &[f64],
    mut ml_index1: self::ml::Big,
) -> self::MlContainsDeclarationDependencyLoop2Result {
    loop {
        return if ml_index1 < (self::ml::Big::from_u128(ml_values1.len() as u128)) {
            {
                let previous = self::ml_array::at(ml_values1, ml_index1.to_index());
                if previous == value {
                    self::MlContainsDeclarationDependencyLoop2Result::MlContainsDeclarationDependencyLoop2Return(true)
                } else {
                    {
                        let ml_index1_2 = ml_index1.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index1 = ml_index1_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlContainsDeclarationDependencyLoop2Result::MlContainsDeclarationDependencyLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptDeclarationDependencyFactsLoop5Result {
    MlAcceptDeclarationDependencyFactsLoop5Done,
    MlAcceptDeclarationDependencyFactsLoop5Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_declaration_dependency_facts_loop5(
    starts: &[f64],
    ml_values3: &[f64],
    mut ml_index3: self::ml::Big,
) -> self::MlAcceptDeclarationDependencyFactsLoop5Result {
    loop {
        return if ml_index3 < (self::ml::Big::from_u128(ml_values3.len() as u128)) {
            {
                let target = self::ml_array::at(ml_values3, ml_index3.to_index());
                if !((target >= 0f64) && (target < (starts.len() as f64)))
                    || ((target % 1f64) != 0f64)
                {
                    self::MlAcceptDeclarationDependencyFactsLoop5Result::MlAcceptDeclarationDependencyFactsLoop5Return(false)
                } else {
                    {
                        let ml_index3_2 = ml_index3.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index3 = ml_index3_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptDeclarationDependencyFactsLoop5Result::MlAcceptDeclarationDependencyFactsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlAcceptDeclarationDependencyFactsLoop4Result {
    MlAcceptDeclarationDependencyFactsLoop4Done(f64),
    MlAcceptDeclarationDependencyFactsLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_declaration_dependency_facts_loop4(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut offset: f64,
    mut node: f64,
) -> self::MlAcceptDeclarationDependencyFactsLoop4Result {
    loop {
        return if node < (starts.len() as f64) {
            {
                let start = self::read_required_number_fact(starts, node);
                {
                    let count = self::read_required_number_fact(counts, node);
                    if (((start != offset) || (count < 0f64)) || ((count % 1f64) != 0f64))
                        || (count > ((targets.len() as f64) - offset))
                    {
                        self::MlAcceptDeclarationDependencyFactsLoop4Result::MlAcceptDeclarationDependencyFactsLoop4Return(false)
                    } else {
                        {
                            let offset_2 = offset + count;
                            {
                                let node_2 = node + 1f64;
                                {
                                    (offset, node) = (offset_2, node_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptDeclarationDependencyFactsLoop4Result::MlAcceptDeclarationDependencyFactsLoop4Done(offset)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop7(
    starts: &[f64],
    mut visited: Vec<f64>,
    mut node: f64,
) -> Vec<f64> {
    loop {
        if node < (starts.len() as f64) {
            {
                let visited_2 = {
                    let (ml_append_right, ml_append_left) = (vec![0f64], visited);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let node_2 = node + 1f64;
                    {
                        (visited, node) = (visited_2, node_2);
                        continue;
                    }
                }
            }
        }
        return { visited };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDependencyComponentsLoop9Result {
    MlReadDeclarationDependencyComponentsLoop9Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop10(
    targets: &[f64],
    visited: &[f64],
    mut pending: Vec<f64>,
    mut start: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge > start {
            {
                let edge_2 = edge - 1f64;
                {
                    let target = self::read_required_number_fact(targets, edge_2);
                    if (self::read_required_number_fact(visited, target)) == 0f64 {
                        {
                            let pending_2 = {
                                let (ml_append_right, ml_append_left) = (vec![target], pending);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                (pending, edge) = (pending_2, edge_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            edge = edge_2;
                            continue;
                        }
                    }
                }
            }
        }
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop9(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut visited: Vec<f64>,
    mut finished: Vec<f64>,
    mut pending: Vec<f64>,
) -> self::MlReadDeclarationDependencyComponentsLoop9Result {
    loop {
        if (pending.len() as f64) > 0f64 {
            {
                let instruction =
                    self::read_required_number_fact(&(pending), ((pending.len() as f64) - 1f64));
                {
                    let pending_2 =
                        self::ml_array::slice(&pending, 0f64, ((pending.len() as f64) - 1f64));
                    if instruction < 0f64 {
                        {
                            let finished_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![((-instruction) - 1f64)], finished);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                (finished, pending) = (finished_2, pending_2);
                                continue;
                            }
                        }
                    }
                    {
                        if (self::read_required_number_fact(&(visited), instruction)) == 0f64 {
                            {
                                let visited_2 = self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::slice(&visited, 0f64, instruction),
                                        vec![1f64],
                                    ),
                                    self::ml_array::slice(
                                        &visited,
                                        (instruction + 1f64),
                                        f64::INFINITY,
                                    ),
                                );
                                {
                                    let pending_3 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![((-instruction) - 1f64)], pending_2);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let start =
                                            self::read_required_number_fact(starts, instruction);
                                        {
                                            let edge = start
                                                + self::read_required_number_fact(
                                                    counts,
                                                    instruction,
                                                );
                                            {
                                                let pending_4 = self::ml_read_declaration_dependency_components_loop10(targets, &(visited_2), pending_3, start, edge);
                                                {
                                                    (visited, pending) = (visited_2, pending_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                pending = pending_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadDeclarationDependencyComponentsLoop9Result::MlReadDeclarationDependencyComponentsLoop9Done(visited, finished)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop8(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut visited: Vec<f64>,
    mut finished: Vec<f64>,
    mut root: f64,
) -> Vec<f64> {
    loop {
        return if root < (starts.len() as f64) {
            if (self::read_required_number_fact(&(visited), root)) == 1f64 {
                {
                    let root_2 = root + 1f64;
                    {
                        root = root_2;
                        continue;
                    }
                }
            }
            {
                {
                    let pending = vec![root];
                    {
                        let ml_s3333 = self::ml_read_declaration_dependency_components_loop9(
                            starts, counts, targets, visited, finished, pending,
                        );
                        match ml_s3333 {
                            self::MlReadDeclarationDependencyComponentsLoop9Result::MlReadDeclarationDependencyComponentsLoop9Done(visited_2, finished_2) => {
                                {
                                    let root_3 = root + 1f64;
                                    {
                                        (visited, finished, root) = (visited_2, finished_2, root_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            finished
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop11(
    starts: &[f64],
    mut assigned: Vec<f64>,
    mut node: f64,
) -> Vec<f64> {
    loop {
        if node < (starts.len() as f64) {
            {
                let assigned_2 = {
                    let (ml_append_right, ml_append_left) = (vec![0f64], assigned);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let node_2 = node + 1f64;
                    {
                        (assigned, node) = (assigned_2, node_2);
                        continue;
                    }
                }
            }
        }
        return { assigned };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop14(
    vertices: &[f64],
    leaders: &[f64],
    mut result: Vec<f64>,
    mut node: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (vertices.len() as f64) {
            if (self::read_required_number_fact(vertices, at)) == node {
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(leaders, at)], result);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
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
        return { result };
    }
}
