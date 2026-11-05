// Generated actual-ABI transport only; shared JS decisions own every dependency/admission result.
use crate::translation::declaration_dependency_rules as rules;

pub(super) fn read_components(starts: &[f64], counts: &[f64], targets: &[f64]) -> Vec<f64> {
    rules::read_declaration_dependency_components(starts, counts, targets)
}

pub(super) fn accept_group(count: f64, ordinary: bool, unique: bool) -> bool {
    rules::accept_recursive_declaration_group(count, ordinary, unique)
}

pub(super) fn accept_publication(
    members: f64,
    declarations: f64,
    signatures: f64,
    flat: bool,
    emitted: bool,
    matching: bool,
) -> bool {
    rules::accept_recursive_declaration_publication(
        members,
        declarations,
        signatures,
        flat,
        emitted,
        matching,
    )
}

pub(super) fn accept_module_scope(forms: &[&str], complete_checked: bool) -> bool {
    let forms = forms
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    rules::accept_declaration_module_scope(&forms, complete_checked)
}

pub(super) fn accept_signature_binding(scope_known: bool, owner_count: f64) -> bool {
    rules::accept_declaration_signature_binding(scope_known, owner_count)
}

pub(super) fn read_source_owners(
    root_starts: &[f64],
    root_ends: &[f64],
    declaration_starts: &[f64],
    declaration_ends: &[f64],
    roles: &[f64],
) -> Vec<f64> {
    rules::read_recursive_declaration_source_owners(
        root_starts,
        root_ends,
        declaration_starts,
        declaration_ends,
        roles,
    )
}

pub(super) fn accept_auxiliary_closure(
    owners: &[f64],
    roles: &[f64],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
) -> bool {
    rules::accept_recursive_declaration_auxiliary_closure(owners, roles, starts, counts, targets)
}

pub(super) fn accept_auxiliary_correspondence(whole: &[f64], member: &[f64]) -> bool {
    rules::accept_recursive_declaration_auxiliary_correspondence(whole, member)
}

pub(super) fn accept_owned_emission(
    roles: &[f64],
    constructor_counts: &[f64],
    blocks: &[f64],
    mappings: &[f64],
    forbidden_targets: &[f64],
) -> bool {
    rules::accept_recursive_declaration_owned_emission(
        roles,
        constructor_counts,
        blocks,
        mappings,
        forbidden_targets,
    )
}

pub(super) fn accept_owned_publication(
    members: f64,
    declarations: f64,
    signatures: f64,
    completed: f64,
    proofs: &[f64],
) -> bool {
    rules::accept_recursive_declaration_owned_publication(
        members,
        declarations,
        signatures,
        completed,
        proofs,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "the complete ordered CST columns are the shared checked interface"
)]
pub(super) fn read_deferred_binding(
    terms: &[&str],
    fields: &[&str],
    parents: &[f64],
    starts: &[f64],
    ends: &[f64],
    texts: &[&str],
    errors: &[f64],
    source_bytes: f64,
) -> Vec<f64> {
    let terms = terms
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let fields = fields
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let texts = texts
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    rules::read_declaration_deferred_binding(
        &terms,
        &fields,
        parents,
        starts,
        ends,
        &texts,
        errors,
        source_bytes,
    )
}

pub(super) fn accept_deferred_module_scope(forms: &[&str]) -> bool {
    let forms = forms
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    rules::accept_declaration_deferred_module_scope(&forms)
}
