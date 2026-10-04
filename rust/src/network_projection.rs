use crate::link_network::{Link, LinkType};

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
        match self {
            Self::Lossless => true,
            Self::ConcreteSyntax => link.metadata().link_type() != Some(LinkType::Semantic),
            Self::AbstractSyntax => !matches!(
                link.metadata().link_type(),
                Some(LinkType::Token | LinkType::Trivia)
            ),
            Self::Semantic => matches!(
                link.metadata().link_type(),
                Some(LinkType::Semantic | LinkType::Concept | LinkType::Type | LinkType::Language)
            ),
        }
    }
}
