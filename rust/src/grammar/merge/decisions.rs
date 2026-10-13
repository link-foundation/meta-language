// Generated from js/src/grammar-merge-decisions.js by
// js/scripts/generate-grammar-merge-decisions.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.

/// Retain each source production when correspondence lacks an equivalence proof.
#[must_use]
pub const fn preserve_source_alternatives(proven: bool, multiple_productions: bool) -> bool {
    !proven && multiple_productions
}

/// Require unique correspondence and compatible semantic meanings.
#[must_use]
pub const fn accept_rule_correspondence(
    multiple_sources: bool,
    unambiguous: bool,
    compatible_concepts: bool,
) -> bool {
    multiple_sources && unambiguous && compatible_concepts
}

/// Preserve lexical token behavior beneath a canonical parser rule.
#[must_use]
pub const fn wrap_lexical_production(source_token: bool, canonical_token: bool) -> bool {
    source_token && !canonical_token
}

// Grammar rule decisions are translated from JavaScript.
