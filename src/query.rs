use crate::link_network::{Link, LinkType};

/// Structural query over links.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkQuery {
    link_type: Option<LinkType>,
    term: Option<String>,
    language: Option<String>,
    named: Option<bool>,
}

impl LinkQuery {
    /// Creates an empty query that matches every link.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts matches to a link type.
    #[must_use]
    pub const fn with_link_type(mut self, link_type: LinkType) -> Self {
        self.link_type = Some(link_type);
        self
    }

    /// Restricts matches to a term.
    #[must_use]
    pub fn with_term(mut self, term: impl Into<String>) -> Self {
        self.term = Some(term.into());
        self
    }

    /// Restricts matches to a language label.
    #[must_use]
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Restricts matches by the named flag.
    #[must_use]
    pub const fn with_named(mut self, named: bool) -> Self {
        self.named = Some(named);
        self
    }

    pub(crate) fn matches(&self, link: &Link) -> bool {
        let metadata = link.metadata();
        self.link_type
            .map_or(true, |link_type| metadata.link_type() == Some(link_type))
            && self
                .term
                .as_deref()
                .map_or(true, |term| metadata.term() == Some(term))
            && self
                .language
                .as_deref()
                .map_or(true, |language| metadata.language() == Some(language))
            && self
                .named
                .map_or(true, |named| metadata.is_named() == named)
    }
}
