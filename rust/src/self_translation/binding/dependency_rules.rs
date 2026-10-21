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
