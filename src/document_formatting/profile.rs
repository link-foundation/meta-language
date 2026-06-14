//! Per-format capability profiles for cross-format document reconstruction.
//!
//! Each supported document format (`txt`, `Markdown`, `HTML`, `PDF`, `DOCX`)
//! exposes a [`LanguageProfile`] over the shared, language-free formatting
//! concept ontology (issue #83). The profile records which formatting concepts
//! the format can represent natively and, for every concept it cannot, the
//! documented lossy fallback applied when the concept is encountered. This is
//! the per-target fidelity report required by issue #86: a document parsed from
//! one format reconstructs into any other when the source uses only concepts
//! both formats support, and unsupported concepts degrade through a declared
//! fallback rather than silent data loss.

use crate::language_profile::LanguageProfile;
use crate::link_network::LinkType;

/// The ordered set of document formats the cross-format reconstruction layer
/// supports, used as both source and target of `reconstruct_text_as`.
pub const DOCUMENT_FORMATS: &[&str] = &["txt", "Markdown", "HTML", "PDF", "DOCX"];

/// The shared formatting concepts considered when reporting cross-format
/// fidelity. Every format profile classifies each of these as either natively
/// supported or carrying a documented lossy fallback.
pub const CROSS_FORMAT_CONCEPTS: &[&str] = &[
    "heading",
    "paragraph",
    "bullet-list",
    "ordered-list",
    "list-item",
    "strong",
    "emphasis",
    "hyperlink",
];

/// Returns the capability profile for a document `format`, or `None` when the
/// format is not one of the cross-format reconstruction targets.
///
/// The returned [`LanguageProfile`] lists the formatting concepts the format
/// represents natively (via [`LanguageProfile::supports_concept`]) and the
/// documented fallback for every concept it cannot
/// (via [`LanguageProfile::concept_fallback`] / [`LanguageProfile::fallbacks`]).
#[must_use]
pub fn document_format_profile(format: &str) -> Option<LanguageProfile> {
    let canonical = canonical_document_format(format)?;
    let profile = base_profile(canonical);
    Some(match canonical {
        "txt" => txt_profile(profile),
        "Markdown" => markdown_profile(profile),
        "HTML" => html_profile(profile),
        "PDF" => pdf_profile(profile),
        "DOCX" => docx_profile(profile),
        _ => unreachable!("canonical_document_format only yields known formats"),
    })
}

/// Canonicalizes a format/language label to one of [`DOCUMENT_FORMATS`].
///
/// Matching is case-insensitive and accepts the common aliases used when a
/// network is parsed (for example `md` for Markdown or `plain-text` for `txt`).
#[must_use]
pub fn canonical_document_format(format: &str) -> Option<&'static str> {
    match format.to_ascii_lowercase().as_str() {
        "txt" | "text" | "plain-text" | "plaintext" => Some("txt"),
        "markdown" | "md" => Some("Markdown"),
        "html" | "htm" => Some("HTML"),
        "pdf" => Some("PDF"),
        "docx" => Some("DOCX"),
        _ => None,
    }
}

fn base_profile(canonical: &str) -> LanguageProfile {
    LanguageProfile::new(canonical, canonical)
        .with_link_type(LinkType::Document)
        .with_link_type(LinkType::Concept)
        .with_link_type(LinkType::Token)
}

fn with_supported<'a>(
    mut profile: LanguageProfile,
    concepts: impl IntoIterator<Item = &'a str>,
) -> LanguageProfile {
    for concept in concepts {
        profile = profile.with_concept(concept);
    }
    profile
}

fn txt_profile(profile: LanguageProfile) -> LanguageProfile {
    with_supported(profile, ["paragraph"])
        .with_concept_fallback(
            "heading",
            "flattened to a plain paragraph (heading level dropped)",
        )
        .with_concept_fallback(
            "bullet-list",
            "flattened to plain lines with a `- ` marker per item",
        )
        .with_concept_fallback(
            "ordered-list",
            "flattened to plain lines with a `N. ` marker per item",
        )
        .with_concept_fallback("list-item", "rendered as a single plain line")
        .with_concept_fallback("strong", "rendered as unstyled plain text")
        .with_concept_fallback("emphasis", "rendered as unstyled plain text")
        .with_concept_fallback("hyperlink", "rendered as its visible text (URL dropped)")
}

fn markdown_profile(profile: LanguageProfile) -> LanguageProfile {
    with_supported(
        profile,
        [
            "heading",
            "paragraph",
            "bullet-list",
            "list-item",
            "strong",
            "emphasis",
            "hyperlink",
        ],
    )
    .with_concept_fallback(
        "ordered-list",
        "rendered with bullet `- ` markers (ordering not preserved by the Markdown profile)",
    )
}

fn html_profile(profile: LanguageProfile) -> LanguageProfile {
    // HTML represents every cross-format concept natively.
    with_supported(profile, CROSS_FORMAT_CONCEPTS.iter().copied())
}

fn pdf_profile(profile: LanguageProfile) -> LanguageProfile {
    with_supported(
        profile,
        [
            "heading",
            "paragraph",
            "bullet-list",
            "ordered-list",
            "list-item",
            "strong",
            "emphasis",
        ],
    )
    .with_concept_fallback(
        "hyperlink",
        "rendered as its visible text, unstyled (URL dropped)",
    )
}

fn docx_profile(profile: LanguageProfile) -> LanguageProfile {
    with_supported(
        profile,
        [
            "heading",
            "paragraph",
            "bullet-list",
            "ordered-list",
            "list-item",
            "strong",
            "emphasis",
        ],
    )
    .with_concept_fallback(
        "hyperlink",
        "rendered as its visible text, unstyled (URL dropped)",
    )
}
