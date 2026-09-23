//! Versioned capability and translation declarations for the four-language surface.

/// Schema revision for parser, emitter, and translation contracts.
pub const LANGUAGE_REPRESENTATION_SCHEMA_VERSION: u32 = 1;

/// Honest capability level for a representation layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepresentationLevel {
    /// Original information is retained exactly.
    Preserved,
    /// A concrete lexical/syntactic structure is available.
    ConcreteSyntax,
    /// Source is retained but its meaning requires a project extension or plugin.
    Opaque,
    /// The runtime does not provide this semantic layer.
    Unavailable,
}

/// Immutable support declaration for a registered language frontend and emitter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LanguageSupport {
    /// Contract schema revision.
    pub schema_version: u32,
    /// Canonical language name.
    pub name: &'static str,
    /// Case-insensitive parser aliases.
    pub aliases: &'static [&'static str],
    /// Exact target language release.
    pub version: &'static str,
    /// Language edition or surface dialect.
    pub edition: &'static str,
    /// Registered file extensions.
    pub extensions: &'static [&'static str],
    /// Original-source preservation level.
    pub source_bytes: RepresentationLevel,
    /// Parser structure level.
    pub concrete_syntax: RepresentationLevel,
    /// Name/scope resolution level.
    pub binding_resolution: RepresentationLevel,
    /// Type or proof elaboration level.
    pub type_elaboration: RepresentationLevel,
    /// Handling of macros, notation, attributes, and plugins.
    pub dynamic_extensions: RepresentationLevel,
    /// Proof/tactic representation level.
    pub proof_syntax: RepresentationLevel,
    /// Registered emitter description.
    pub emitter: &'static str,
}

const fn support(
    name: &'static str,
    aliases: &'static [&'static str],
    version: &'static str,
    edition: &'static str,
    extensions: &'static [&'static str],
    proof_syntax: RepresentationLevel,
) -> LanguageSupport {
    LanguageSupport {
        schema_version: LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
        name,
        aliases,
        version,
        edition,
        extensions,
        source_bytes: RepresentationLevel::Preserved,
        concrete_syntax: RepresentationLevel::ConcreteSyntax,
        binding_resolution: RepresentationLevel::Unavailable,
        type_elaboration: RepresentationLevel::Unavailable,
        dynamic_extensions: RepresentationLevel::Opaque,
        proof_syntax,
        emitter: "ordered source-token emitter",
    }
}

/// The four built-in parser/emitter capability declarations.
pub const FOUR_LANGUAGE_SUPPORT: [LanguageSupport; 4] = [
    support(
        "JavaScript",
        &["javascript", "js", "ecmascript"],
        "ECMAScript 2026",
        "ECMA-262, 17th edition",
        &[".js", ".mjs", ".cjs"],
        RepresentationLevel::Unavailable,
    ),
    support(
        "Rust",
        &["rust", "rs"],
        "Rust 1.98.1",
        "2024",
        &[".rs"],
        RepresentationLevel::Unavailable,
    ),
    support(
        "Lean",
        &["lean", "lean4"],
        "Lean 4.34.0",
        "Lean 4",
        &[".lean"],
        RepresentationLevel::Opaque,
    ),
    support(
        "Rocq",
        &["rocq", "coq"],
        "Rocq 9.3.0",
        "Vernacular",
        &[".v"],
        RepresentationLevel::Opaque,
    ),
];

/// Returns the capability declaration for a canonical name or alias.
#[must_use]
pub fn language_support(language: &str) -> Option<&'static LanguageSupport> {
    FOUR_LANGUAGE_SUPPORT.iter().find(|support| {
        support
            .aliases
            .iter()
            .any(|alias| alias.eq_ignore_ascii_case(language))
    })
}

/// Translation-hook result when no semantics-preserving implementation is registered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslationSupport {
    /// Translation must stop and return the contract's precise obligation.
    UnsupportedObligation,
}

/// One directed source-to-target translation contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationContract {
    /// Contract schema revision.
    pub schema_version: u32,
    /// Canonical source language.
    pub source: &'static str,
    /// Canonical target language.
    pub target: &'static str,
    /// Current support state.
    pub support: TranslationSupport,
    /// Observable behavior currently represented.
    pub observation: &'static str,
    /// Runtime required to validate target artifacts.
    pub required_runtime: &'static str,
    /// Registered non-native encoding, if any.
    pub encoding: &'static str,
    /// Assumptions made by the translation.
    pub assumptions: &'static [&'static str],
    /// Precise reason translation cannot proceed.
    pub obligation: String,
}

/// Returns all 12 directed hooks among JavaScript, Rust, Lean, and Rocq.
#[must_use]
pub fn translation_contracts() -> Vec<TranslationContract> {
    FOUR_LANGUAGE_SUPPORT
        .iter()
        .flat_map(|source| {
            FOUR_LANGUAGE_SUPPORT
                .iter()
                .filter(move |target| target.name != source.name)
                .map(move |target| contract(source, target))
        })
        .collect()
}

/// Returns one directed translation hook, excluding same-language emission.
#[must_use]
pub fn translation_contract(
    source_language: &str,
    target_language: &str,
) -> Option<TranslationContract> {
    let source = language_support(source_language)?;
    let target = language_support(target_language)?;
    (source.name != target.name).then(|| contract(source, target))
}

fn contract(source: &LanguageSupport, target: &LanguageSupport) -> TranslationContract {
    TranslationContract {
        schema_version: LANGUAGE_REPRESENTATION_SCHEMA_VERSION,
        source: source.name,
        target: target.name,
        support: TranslationSupport::UnsupportedObligation,
        observation: "source bytes and source-runtime concrete syntax; no resolved semantics",
        required_runtime: runtime_for(target.name),
        encoding: "none registered",
        assumptions: &[],
        obligation: format!(
            "No semantic-preservation proof or explicit encoding is registered for {} → {}; translation must stop instead of relabelling source text.",
            source.name, target.name
        ),
    }
}

fn runtime_for(language: &str) -> &'static str {
    match language {
        "Lean" => "Lean 4.34.0 kernel and project environment",
        "Rocq" => "Rocq 9.3.0 kernel and project environment",
        "Rust" => "Rust 1.98.1, edition 2024",
        _ => "ECMAScript 2026 host",
    }
}
