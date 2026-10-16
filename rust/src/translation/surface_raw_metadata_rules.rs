// Generated from the checked JavaScript raw surface metadata decisions. Do not edit.
/// Read a shared raw surface metadata decision.
#[must_use]
pub fn accept_surface_raw_metadata_field(context: &str, field: &str) -> bool {
    (((context == "expression") && (field == "parenthesized"))
        || ((context == "program") && (field == "referenceSemantics")))
        || ((context == "index") && ((field == "optional") || (field == "provenInBounds")))
}

/// Read a shared raw surface metadata decision.
#[must_use]
pub fn accept_surface_raw_metadata_form(form: &str) -> bool {
    (form == "boolean") || (form == "null")
}

/// Read a shared raw surface metadata decision.
#[must_use]
pub fn accept_shared_surface_reference_kind(kind: &str) -> bool {
    (((kind == "array") || (kind == "record")) || (kind == "map")) || (kind == "set")
}

/// Read a shared raw surface metadata decision.
#[must_use]
pub fn select_surface_reference_semantics(required: bool, structural: bool, enabled: bool) -> bool {
    required || (enabled && structural)
}
