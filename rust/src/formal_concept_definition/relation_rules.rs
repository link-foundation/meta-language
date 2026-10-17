// Generated from js/src/formal-relation-rules.js by
// js/scripts/generate-formal-relation-rules.mjs. Do not edit by hand.
// Emitter styles: parity/self-translation/formal-relation-rules-decorators.lino.
/// Admit the existing formal labeled relation model.
#[must_use]
pub fn accept_formal_relation_label_identity(
    label_kind: &str,
    label_definition: &str,
    label_flags: (bool, bool, bool, bool),
) -> bool {
    let (label_has_term, label_has_namespace, label_named, label_self_point) = label_flags;
    label_kind == "Field"
        && label_definition == "A field label names a relation between links."
        && label_has_term
        && !label_has_namespace
        && label_named
        && label_self_point
}

/// Admit the existing formal labeled relation model.
#[must_use]
pub fn accept_formal_relation_edge_identity(
    relation_kind: &str,
    relation_flags: (bool, bool, bool, bool),
    relation_arity: f64,
) -> bool {
    let (relation_has_namespace, relation_has_term, relation_has_definition, relation_named) =
        relation_flags;
    relation_kind == "Field"
        && !relation_has_namespace
        && !relation_has_term
        && !relation_has_definition
        && !relation_named
        && relation_arity.to_bits() == 3.0_f64.to_bits()
}

/// Admit the existing formal labeled relation model.
#[must_use]
pub const fn accept_formal_relation_identity(
    label_admitted: bool,
    relation_admitted: bool,
    same_label_identity: bool,
) -> bool {
    label_admitted && relation_admitted && same_label_identity
}
