//! Grammar links recorded on each parsed language.

use super::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::language_catalog::{GrammarProvenance, grammar_provenance};

impl LinkNetwork {
    /// Records the grammars that parse `language` by default as Grammar links
    /// below its Language link: term `<id>@<version>`, definition
    /// `sha256:<parser digest>`. Recording the same grammar twice is a no-op.
    pub(crate) fn record_grammars(&mut self, language_link: LinkId, language: &str) {
        for grammar in grammar_provenance(language) {
            let term = format!("{}@{}", grammar.id, grammar.version);
            let recorded = self.links().any(|link| {
                link.metadata().link_type() == Some(LinkType::Grammar)
                    && link.references() == [language_link]
                    && link.metadata().term() == Some(term.as_str())
            });
            if !recorded {
                self.insert_link(
                    [language_link],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Grammar)
                        .with_named(true)
                        .with_term(term)
                        .with_definition(format!("sha256:{}", grammar.parser_sha256))
                        .with_language(language),
                );
            }
        }
    }

    /// Grammars recorded for the languages this network parsed, in insertion
    /// order, as `(language, grammar)` pairs.
    #[must_use]
    pub fn parse_grammars(&self) -> Vec<(String, GrammarProvenance)> {
        self.links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Grammar))
            .filter_map(|link| {
                let [language_link] = link.references() else {
                    return None;
                };
                let language = self.link(*language_link)?;
                if language.metadata().link_type() != Some(LinkType::Language) {
                    return None;
                }
                let (id, version) = link.metadata().term()?.split_once('@')?;
                let parser_sha256 = link.metadata().definition()?.strip_prefix("sha256:")?;
                Some((
                    language.metadata().term()?.to_string(),
                    GrammarProvenance {
                        id: id.to_string(),
                        version: version.to_string(),
                        parser_sha256: parser_sha256.to_string(),
                    },
                ))
            })
            .collect()
    }
}
