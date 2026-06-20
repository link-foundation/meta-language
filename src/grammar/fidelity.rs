//! Per-format capability profiles for grammar round-trip fidelity.
//!
//! Each supported grammar notation exposes a [`LanguageProfile`] over the
//! grammar IR construct vocabulary. Constructs are classified as lossless or
//! equivalent when the target notation represents them without a documented
//! lossy fallback, and lossy when the emitter must degrade, synthesize helper
//! productions, drop metadata, or reject the construct with an explicit
//! unsupported-construct error.

use std::collections::{BTreeMap, BTreeSet};

use crate::language_profile::LanguageProfile;
use crate::link_network::LinkType;

/// Grammar formats with a fidelity profile.
///
/// The matrix starts with BNF because it is the first import/emission pair this
/// issue depends on. Later importer/emitter issues can add rows by extending
/// this list and adding a profile function branch.
pub const GRAMMAR_FORMATS: &[&str] = &["bnf"];

/// The grammar IR construct vocabulary classified by the fidelity matrix.
///
/// Every [`GrammarFormatProfile`] must classify each construct as either
/// lossless/equivalent support or exactly one documented lossy fallback.
pub const GRAMMAR_CONSTRUCTS: &[&str] = &[
    "empty",
    "sequence",
    "ordered-choice",
    "unordered-choice",
    "optional",
    "zero-or-more",
    "one-or-more",
    "repeat-range",
    "char-range",
    "char-class",
    "any-char",
    "terminal",
    "case-insensitive-terminal",
    "non-terminal",
    "and-predicate",
    "not-predicate",
    "capture",
    "rule-kind-atomic",
    "rule-kind-silent",
    "rule-kind-token",
];

/// Round-trip fidelity level for one construct in one grammar format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarFidelityLevel {
    /// The construct is represented natively and survives same-format
    /// import/emit/re-import without a semantic fallback.
    Lossless,
    /// The construct is represented by a semantically equivalent spelling or
    /// normalization.
    Equivalent,
    /// The construct requires a documented fallback, metadata drop, helper
    /// expansion, or explicit unsupported-construct error.
    Lossy,
}

impl GrammarFidelityLevel {
    /// Markdown cell symbol used by `docs/grammar/fidelity.md`.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Lossless => "✅",
            Self::Equivalent => "≈",
            Self::Lossy => "⚠️",
        }
    }
}

/// Capability profile for one grammar notation.
///
/// The embedded [`LanguageProfile`] carries the support-or-fallback invariant
/// used by the document fidelity matrix. `equivalent_constructs` marks
/// supported constructs whose round trip is semantically equivalent but not
/// byte/canonical-form lossless.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarFormatProfile {
    format: &'static str,
    profile: LanguageProfile,
    equivalent_constructs: BTreeSet<String>,
}

impl GrammarFormatProfile {
    /// Creates a grammar format profile around a [`LanguageProfile`].
    #[must_use]
    pub const fn new(format: &'static str, profile: LanguageProfile) -> Self {
        Self {
            format,
            profile,
            equivalent_constructs: BTreeSet::new(),
        }
    }

    /// Returns this profile with a lossless construct.
    #[must_use]
    pub fn with_lossless_construct(mut self, construct: impl Into<String>) -> Self {
        self.profile = self.profile.with_concept(construct);
        self
    }

    /// Returns this profile with an equivalent construct.
    #[must_use]
    pub fn with_equivalent_construct(mut self, construct: impl Into<String>) -> Self {
        let construct = construct.into();
        self.profile = self.profile.with_concept(construct.clone());
        self.equivalent_constructs.insert(construct);
        self
    }

    /// Returns this profile with a documented lossy fallback for a construct.
    #[must_use]
    pub fn with_lossy_fallback(
        mut self,
        construct: impl Into<String>,
        fallback: impl Into<String>,
    ) -> Self {
        self.profile = self.profile.with_concept_fallback(construct, fallback);
        self
    }

    /// Canonical format label for this profile.
    #[must_use]
    pub const fn format(&self) -> &'static str {
        self.format
    }

    /// Underlying language profile.
    #[must_use]
    pub const fn language_profile(&self) -> &LanguageProfile {
        &self.profile
    }

    /// Lossy fallback table keyed by construct id.
    #[must_use]
    pub const fn fallbacks(&self) -> &BTreeMap<String, String> {
        self.profile.fallbacks()
    }

    /// Constructs represented through equivalent spelling or normalization.
    #[must_use]
    pub const fn equivalent_constructs(&self) -> &BTreeSet<String> {
        &self.equivalent_constructs
    }

    /// Whether this format represents a construct without a lossy fallback.
    #[must_use]
    pub fn supports_construct(&self, construct: &str) -> bool {
        self.profile.supports_concept(construct)
    }

    /// Documented lossy fallback for a construct this format cannot represent
    /// natively.
    #[must_use]
    pub fn construct_fallback(&self, construct: &str) -> Option<&str> {
        self.profile.concept_fallback(construct)
    }

    /// Fidelity level for a construct, or `None` when the construct is outside
    /// this profile's vocabulary.
    #[must_use]
    pub fn construct_fidelity(&self, construct: &str) -> Option<GrammarFidelityLevel> {
        if self.supports_construct(construct) {
            if self.equivalent_constructs.contains(construct) {
                Some(GrammarFidelityLevel::Equivalent)
            } else {
                Some(GrammarFidelityLevel::Lossless)
            }
        } else if self.construct_fallback(construct).is_some() {
            Some(GrammarFidelityLevel::Lossy)
        } else {
            None
        }
    }
}

/// Returns the capability profile for a grammar `format`, or `None` when the
/// format has no fidelity row yet.
#[must_use]
pub fn grammar_format_profile(format: &str) -> Option<GrammarFormatProfile> {
    let canonical = canonical_grammar_format(format)?;
    Some(match canonical {
        "bnf" => bnf_profile(),
        _ => unreachable!("canonical_grammar_format only yields known formats"),
    })
}

/// Canonicalizes a grammar format label to one of [`GRAMMAR_FORMATS`].
#[must_use]
pub fn canonical_grammar_format(format: &str) -> Option<&'static str> {
    match format.to_ascii_lowercase().as_str() {
        "bnf" | "classic-bnf" | "classic bnf" | "backus-naur form" | "backus naur form" => {
            Some("bnf")
        }
        _ => None,
    }
}

fn base_profile(format: &'static str, name: &'static str) -> GrammarFormatProfile {
    GrammarFormatProfile::new(
        format,
        LanguageProfile::new(name, format)
            .with_link_type(LinkType::Grammar)
            .with_link_type(LinkType::Concept)
            .with_link_type(LinkType::Token),
    )
}

fn with_lossless_constructs<'a>(
    mut profile: GrammarFormatProfile,
    constructs: impl IntoIterator<Item = &'a str>,
) -> GrammarFormatProfile {
    for construct in constructs {
        profile = profile.with_lossless_construct(construct);
    }
    profile
}

fn bnf_profile() -> GrammarFormatProfile {
    with_lossless_constructs(
        base_profile("bnf", "Backus-Naur Form"),
        [
            "empty",
            "sequence",
            "unordered-choice",
            "terminal",
            "non-terminal",
        ],
    )
    .with_lossy_fallback(
        "ordered-choice",
        "emitted as an unordered BNF alternative; priority semantics are not preserved",
    )
    .with_lossy_fallback(
        "optional",
        "emitted through a synthetic helper production with an empty alternative",
    )
    .with_lossy_fallback(
        "zero-or-more",
        "emitted through a recursive synthetic helper production with an empty alternative",
    )
    .with_lossy_fallback(
        "one-or-more",
        "emitted through a recursive synthetic helper production plus one required item",
    )
    .with_lossy_fallback(
        "repeat-range",
        "emitted as required occurrences plus optional or recursive synthetic helper productions",
    )
    .with_lossy_fallback(
        "char-range",
        "expanded to a synthetic helper production enumerating each character when the range is bounded",
    )
    .with_lossy_fallback(
        "char-class",
        "expanded to a synthetic helper production for finite non-negated classes; unsupported classes are rejected",
    )
    .with_lossy_fallback(
        "any-char",
        "unsupported by BNF emission and rejected instead of silently broadening the language",
    )
    .with_lossy_fallback(
        "case-insensitive-terminal",
        "emitted as a case-sensitive literal and reported as lossy",
    )
    .with_lossy_fallback(
        "and-predicate",
        "unsupported by BNF emission and rejected because lookahead has no BNF equivalent",
    )
    .with_lossy_fallback(
        "not-predicate",
        "unsupported by BNF emission and rejected because lookahead has no BNF equivalent",
    )
    .with_lossy_fallback(
        "capture",
        "emitted as the captured expression while dropping the capture label",
    )
    .with_lossy_fallback(
        "rule-kind-atomic",
        "emitted as a normal BNF production; rule-kind metadata is dropped",
    )
    .with_lossy_fallback(
        "rule-kind-silent",
        "emitted as a normal BNF production; rule-kind metadata is dropped",
    )
    .with_lossy_fallback(
        "rule-kind-token",
        "emitted as a normal BNF production; rule-kind metadata is dropped",
    )
}
