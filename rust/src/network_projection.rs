use crate::link_network::{Link, LinkType};
use crate::translation::network_projection_rules::accept_network_projection_link;

/// View of a links network with lower-level data optionally stripped away.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkProjection {
    /// Full lossless network, including all source-preservation links.
    Lossless,
    /// Concrete syntax view, including tokens, trivia, fields, and spans.
    ConcreteSyntax,
    /// Abstract syntax view, excluding lossless token and trivia links.
    AbstractSyntax,
    /// Meaning-focused view, keeping semantic, concept, type, and language links.
    Semantic,
}

impl NetworkProjection {
    /// Human-readable projection name.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Lossless => "lossless",
            Self::ConcreteSyntax => "concrete syntax",
            Self::AbstractSyntax => "abstract syntax",
            Self::Semantic => "semantic",
        }
    }

    pub(crate) fn includes(self, link: &Link) -> bool {
        // Mechanical enum transport; the complete shared rule owns membership.
        let projection = match self {
            Self::Lossless => "Lossless",
            Self::ConcreteSyntax => "ConcreteSyntax",
            Self::AbstractSyntax => "AbstractSyntax",
            Self::Semantic => "Semantic",
        };
        let role = match link.metadata().link_type() {
            None => "",
            Some(LinkType::Link) => "Dynamic",
            Some(LinkType::Reference) => "Reference",
            Some(LinkType::Relation) => "Relation",
            Some(LinkType::Language) => "Language",
            Some(LinkType::Grammar) => "Grammar",
            Some(LinkType::Type) => "Type",
            Some(LinkType::Concept) => "Concept",
            Some(LinkType::Syntax) => "Syntax",
            Some(LinkType::Field) => "Field",
            Some(LinkType::Trivia) => "Trivia",
            Some(LinkType::Token) => "SourceToken",
            Some(LinkType::Document) => "Document",
            Some(LinkType::Semantic) => "Semantic",
            Some(LinkType::Region) => "Region",
            Some(LinkType::Object) => "Object",
        };
        accept_network_projection_link(projection, role)
    }
}
