use crate::{
    FormalizationLevel, LinkNetwork, LinkType, NaturalizationDirection, ParseConfiguration,
    TranslationRuleRegistry, TranslationRuleSet,
};

impl LinkNetwork {
    /// Reconstructs text for a target language or formalization level.
    ///
    /// Natural same-language reconstruction returns the original byte-exact
    /// token stream. When semantic proposition links are available, target
    /// natural-language text and configured formal representations are rendered
    /// through the shared concept mappings.
    #[must_use]
    pub fn reconstruct_text_as(
        &self,
        target_language: &str,
        configuration: ParseConfiguration,
    ) -> String {
        let source = self.reconstruct_text();
        if self.is_document_language(target_language)
            && configuration.formalization_level() == FormalizationLevel::Natural
            && configuration.naturalization_direction() == NaturalizationDirection::Naturalize
        {
            return source;
        }

        self.reconstruct_text_as_with_rules(
            target_language,
            configuration,
            &TranslationRuleSet::statehood_demo(),
        )
    }

    /// Reconstructs text using a caller-supplied translation rule set.
    #[must_use]
    pub fn reconstruct_text_as_with_rules(
        &self,
        target_language: &str,
        configuration: ParseConfiguration,
        rule_set: &TranslationRuleSet,
    ) -> String {
        let source = self.reconstruct_text();
        if self.is_document_language(target_language)
            && configuration.formalization_level() == FormalizationLevel::Natural
            && configuration.naturalization_direction() == NaturalizationDirection::Naturalize
        {
            return source;
        }

        rule_set
            .render(self, target_language, configuration)
            .unwrap_or(source)
    }

    /// Reconstructs text through the active rule set in a registry.
    #[must_use]
    pub fn reconstruct_text_as_with_registry(
        &self,
        target_language: &str,
        configuration: ParseConfiguration,
        registry: &TranslationRuleRegistry,
    ) -> String {
        registry.active_rule_set().map_or_else(
            || self.reconstruct_text(),
            |rule_set| {
                self.reconstruct_text_as_with_rules(target_language, configuration, rule_set)
            },
        )
    }

    /// Reconstructs text and records diagnostic links when no rule can render it.
    pub fn reconstruct_text_as_with_rules_mut(
        &mut self,
        target_language: &str,
        configuration: ParseConfiguration,
        rule_set: &TranslationRuleSet,
    ) -> String {
        let source = self.reconstruct_text();
        if self.is_document_language(target_language)
            && configuration.formalization_level() == FormalizationLevel::Natural
            && configuration.naturalization_direction() == NaturalizationDirection::Naturalize
        {
            return source;
        }

        if let Some(rendered) = rule_set.render(self, target_language, configuration) {
            return rendered;
        }

        self.insert_missing_translation_diagnostics(target_language);
        source
    }

    fn insert_missing_translation_diagnostics(&mut self, target_language: &str) {
        let unmatched = self
            .links()
            .filter(|link| {
                link.metadata().link_type() == Some(LinkType::Semantic)
                    && !link
                        .metadata()
                        .term()
                        .is_some_and(|term| term.starts_with("translation-rule:"))
            })
            .map(|link| {
                (
                    link.id(),
                    link.metadata()
                        .term()
                        .unwrap_or("semantic link")
                        .to_string(),
                )
            })
            .collect::<Vec<_>>();

        for (link_id, term) in unmatched {
            if self.has_missing_translation_diagnostic(link_id, target_language) {
                continue;
            }
            self.insert_link(
                [link_id],
                crate::LinkMetadata::new()
                    .with_link_type(LinkType::Semantic)
                    .with_named(true)
                    .with_term("translation-rule:missing")
                    .with_language(target_language)
                    .with_definition(format!(
                        "Missing translation rule for `{term}` targeting `{target_language}`."
                    )),
            );
        }
    }

    fn has_missing_translation_diagnostic(
        &self,
        link_id: crate::LinkId,
        target_language: &str,
    ) -> bool {
        self.links().any(|link| {
            link.references() == [link_id]
                && link.metadata().link_type() == Some(LinkType::Semantic)
                && link.metadata().term() == Some("translation-rule:missing")
                && link.metadata().language() == Some(target_language)
        })
    }

    fn is_document_language(&self, target_language: &str) -> bool {
        self.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Document)
                && languages_match(link.metadata().language(), target_language)
        })
    }
}

fn canonical_reconstruction_language(language: &str) -> Option<&'static str> {
    match language.to_ascii_lowercase().as_str() {
        "english" | "en" => Some("English"),
        "russian" | "ru" => Some("Russian"),
        _ => None,
    }
}

fn languages_match(source_language: Option<&str>, target_language: &str) -> bool {
    let Some(source_language) = source_language else {
        return false;
    };

    source_language == target_language
        || canonical_reconstruction_language(source_language)
            .zip(canonical_reconstruction_language(target_language))
            .is_some_and(|(source, target)| source == target)
}
