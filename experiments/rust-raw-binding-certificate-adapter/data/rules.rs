// Experimental automatic translation; no native certificate host.
/// Isolated raw binding certificate decision.
#[must_use]
pub fn accept_rust_raw_binding_certificate_source(language: &str, independent: bool) -> bool {
    (language == "Rust") && independent
}

/// Isolated raw binding certificate decision.
#[must_use]
pub fn accept_rust_raw_binding_certificate_context(model: &str) -> bool {
    model == "program owned Rust raw binding source context 1"
}

/// Isolated raw binding certificate decision.
#[must_use]
pub const fn accept_rust_raw_binding_certificate_roots(facet: bool, context: bool) -> bool {
    facet && context
}

/// Isolated raw binding certificate decision.
#[must_use]
pub const fn accept_rust_raw_binding_certificate_owners(worker: bool, extent: bool) -> bool {
    worker && extent
}
