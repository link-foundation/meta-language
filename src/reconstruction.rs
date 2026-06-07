use crate::{
    FormalizationLevel, LinkNetwork, LinkType, NaturalizationDirection, ParseConfiguration,
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

        if !self.has_statehood_proposition() {
            return source;
        }

        self.reconstruct_statehood(target_language, configuration, &source)
            .unwrap_or(source)
    }

    fn reconstruct_concept_for_language(&self, concept: &str, language: &str) -> Option<&str> {
        self.reconstruct_concept(concept, language).or_else(|| {
            canonical_reconstruction_language(language)
                .and_then(|canonical| self.reconstruct_concept(concept, canonical))
        })
    }

    fn reconstruct_statehood(
        &self,
        target_language: &str,
        configuration: ParseConfiguration,
        source: &str,
    ) -> Option<String> {
        let level = match (
            configuration.naturalization_direction(),
            configuration.formalization_level(),
        ) {
            (NaturalizationDirection::Formalize, FormalizationLevel::Natural) => {
                FormalizationLevel::Lexical
            }
            (_, level) => level,
        };

        let body = match level {
            FormalizationLevel::Natural => self
                .reconstruct_concept_for_language("statehood", target_language)?
                .to_string(),
            FormalizationLevel::Lexical => {
                let subject = self
                    .reconstruct_concept_for_language("Q782", target_language)
                    .unwrap_or("Q782");
                let object = self
                    .reconstruct_concept_for_language("Q35657", target_language)
                    .unwrap_or("Q35657");
                format!("statehood({subject}, {object})")
            }
            FormalizationLevel::Concept => "statehood(Q782, Q35657)".to_string(),
            FormalizationLevel::Logical => {
                "(proposition: statehood (subject: Q782) (object: Q35657) (truth: true))"
                    .to_string()
            }
        };

        Some(with_source_trailing_newline(body, source))
    }

    fn has_statehood_proposition(&self) -> bool {
        self.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Semantic)
                && link.metadata().term() == Some("proposition:statehood")
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

fn with_source_trailing_newline(mut body: String, source: &str) -> String {
    if source.ends_with('\n') {
        body.push('\n');
    }
    body
}
