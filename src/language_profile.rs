use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::{LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, TranslationRuleSet};

const PROFILE_TERM: &str = "language-profile";
const PROFILE_LINK_TYPE_TERM: &str = "language-profile:link-type";
const PROFILE_CONCEPT_TERM: &str = "language-profile:concept";
const PROFILE_TRANSLATION_RULE_TERM: &str = "language-profile:translation-rule";
const PROFILE_DIAGNOSTIC_TERM: &str = "language-profile:unsupported-feature";

/// Per-language capability profile for restricting transforms to supported features.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageProfile {
    name: String,
    language: String,
    link_types: BTreeSet<LinkType>,
    concepts: BTreeSet<String>,
    translation_rules: BTreeSet<String>,
}

impl LanguageProfile {
    /// Creates an empty profile for a target language.
    #[must_use]
    pub fn new(name: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            language: language.into(),
            link_types: BTreeSet::new(),
            concepts: BTreeSet::new(),
            translation_rules: BTreeSet::new(),
        }
    }

    /// Built-in JavaScript same-language profile.
    #[must_use]
    pub fn javascript() -> Self {
        let mut profile = Self::new("JavaScript", "JavaScript");
        for link_type in [
            LinkType::Link,
            LinkType::Reference,
            LinkType::Relation,
            LinkType::Language,
            LinkType::Grammar,
            LinkType::Type,
            LinkType::Concept,
            LinkType::Syntax,
            LinkType::Field,
            LinkType::Trivia,
            LinkType::Token,
            LinkType::Document,
            LinkType::Semantic,
            LinkType::Region,
            LinkType::Object,
        ] {
            profile = profile.with_link_type(link_type);
        }
        profile
    }

    /// Looks up a built-in profile by name.
    #[must_use]
    pub fn builtin(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "javascript" | "js" => Some(Self::javascript()),
            _ => None,
        }
    }

    /// Computes a profile domain from a translation rule set.
    ///
    /// Rule query link-type filters become supported link types, query term
    /// filters become supported concept/feature terms, and every rule name is
    /// recorded as a supported translation rule.
    #[must_use]
    pub fn from_rule_set(
        name: impl Into<String>,
        language: impl Into<String>,
        rule_set: &TranslationRuleSet,
    ) -> Self {
        let mut profile = Self::new(name, language);
        for rule in rule_set.rules() {
            profile = profile.with_translation_rule(rule.name());
            if let Some(link_type) = rule.query().link_type_filter() {
                profile = profile.with_link_type(link_type);
            }
            if let Some(term) = rule.query().term_filter() {
                profile = profile.with_concept(term);
            }
        }
        profile
    }

    /// Profile name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Target language this profile constrains.
    #[must_use]
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Supported link types.
    #[must_use]
    pub const fn link_types(&self) -> &BTreeSet<LinkType> {
        &self.link_types
    }

    /// Supported concept or feature terms.
    #[must_use]
    pub const fn concepts(&self) -> &BTreeSet<String> {
        &self.concepts
    }

    /// Supported translation rule names.
    #[must_use]
    pub const fn translation_rules(&self) -> &BTreeSet<String> {
        &self.translation_rules
    }

    /// Returns a copy with a supported link type.
    #[must_use]
    pub fn with_link_type(mut self, link_type: LinkType) -> Self {
        self.link_types.insert(link_type);
        self
    }

    /// Returns a copy with a supported concept or feature term.
    #[must_use]
    pub fn with_concept(mut self, concept: impl Into<String>) -> Self {
        self.concepts.insert(concept.into());
        self
    }

    /// Returns a copy with a supported translation rule name.
    #[must_use]
    pub fn with_translation_rule(mut self, rule: impl Into<String>) -> Self {
        self.translation_rules.insert(rule.into());
        self
    }

    /// Whether this profile supports a link type.
    #[must_use]
    pub fn supports_link_type(&self, link_type: LinkType) -> bool {
        self.link_types.contains(&link_type)
    }

    /// Whether this profile supports a concept or feature term.
    #[must_use]
    pub fn supports_concept(&self, concept: &str) -> bool {
        self.concepts.contains(concept)
    }

    /// Whether this profile supports a translation rule name.
    #[must_use]
    pub fn supports_translation_rule(&self, rule: &str) -> bool {
        self.translation_rules.contains(rule)
    }

    /// Declares this profile as queryable links inside a network.
    pub fn declare_in(&self, network: &mut LinkNetwork) -> LanguageProfileLinks {
        let profile = self.profile_link(network).unwrap_or_else(|| {
            network.insert_link(
                [],
                LinkMetadata::new()
                    .with_link_type(LinkType::Semantic)
                    .with_named(true)
                    .with_term(PROFILE_TERM)
                    .with_language(&self.language)
                    .with_definition(&self.name),
            )
        });
        let mut capabilities = Vec::new();

        for link_type in &self.link_types {
            capabilities.push(self.ensure_capability_link(
                network,
                profile,
                PROFILE_LINK_TYPE_TERM,
                &link_type.to_string(),
            ));
        }
        for concept in &self.concepts {
            capabilities.push(self.ensure_capability_link(
                network,
                profile,
                PROFILE_CONCEPT_TERM,
                concept,
            ));
        }
        for rule in &self.translation_rules {
            capabilities.push(self.ensure_capability_link(
                network,
                profile,
                PROFILE_TRANSLATION_RULE_TERM,
                rule,
            ));
        }

        LanguageProfileLinks {
            profile,
            capabilities,
        }
    }

    /// Validates that all typed links in a network stay inside this profile.
    ///
    /// # Errors
    ///
    /// Returns [`LanguageProfileViolation`] for the first unsupported link
    /// type found in identifier order.
    pub fn validate_network(&self, network: &LinkNetwork) -> Result<(), LanguageProfileViolation> {
        for link in network.links() {
            if let Some(link_type) = link.metadata().link_type() {
                if !self.supports_link_type(link_type) {
                    return Err(LanguageProfileViolation::new(
                        format!("link type `{link_type}`"),
                        format!(
                            "Profile `{}` for `{}` does not support link type `{link_type}`.",
                            self.name, self.language
                        ),
                    ));
                }
            }

            if self.concepts.is_empty()
                || !matches!(
                    link.metadata().link_type(),
                    Some(LinkType::Concept | LinkType::Semantic)
                )
            {
                continue;
            }
            let Some(term) = link.metadata().term() else {
                continue;
            };
            if is_profile_control_term(term) || self.supports_concept(term) {
                continue;
            }
            return Err(LanguageProfileViolation::new(
                format!("concept `{term}`"),
                format!(
                    "Profile `{}` for `{}` does not support concept `{term}`.",
                    self.name, self.language
                ),
            ));
        }
        Ok(())
    }

    pub(crate) fn validate_transform_result(
        &self,
        network: &LinkNetwork,
    ) -> Result<(), LanguageProfileViolation> {
        self.validate_network(network)?;

        let source = network.reconstruct_text();
        if source.is_empty() {
            return Ok(());
        }

        let parsed = LinkNetwork::parse(&source, &self.language, ParseConfiguration::default());
        let report = parsed.verify_full_match(None);
        if report.issues().is_empty() {
            Ok(())
        } else {
            Err(LanguageProfileViolation::new(
                format!("{} syntax", self.language),
                format!(
                    "Profile `{}` for `{}` rejects source text that is not valid {}.",
                    self.name, self.language, self.language
                ),
            ))
        }
    }

    pub(crate) fn insert_diagnostic(
        &self,
        network: &mut LinkNetwork,
        violation: &LanguageProfileViolation,
        subject: Option<LinkId>,
    ) -> LinkId {
        let profile = self.declare_in(network).profile();
        let metadata = LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term(PROFILE_DIAGNOSTIC_TERM)
            .with_language(&self.language)
            .with_definition(violation.to_string());

        match subject {
            Some(subject) => network.insert_link([profile, subject], metadata),
            None => network.insert_link([profile], metadata),
        }
    }

    fn profile_link(&self, network: &LinkNetwork) -> Option<LinkId> {
        network
            .links()
            .find(|link| {
                link.metadata().link_type() == Some(LinkType::Semantic)
                    && link.metadata().term() == Some(PROFILE_TERM)
                    && link.metadata().language() == Some(self.language())
                    && link.metadata().definition() == Some(self.name())
            })
            .map(crate::Link::id)
    }

    fn ensure_capability_link(
        &self,
        network: &mut LinkNetwork,
        profile: LinkId,
        term: &str,
        definition: &str,
    ) -> LinkId {
        if let Some(existing) = network
            .links()
            .find(|link| {
                link.references() == [profile]
                    && link.metadata().link_type() == Some(LinkType::Semantic)
                    && link.metadata().term() == Some(term)
                    && link.metadata().language() == Some(self.language())
                    && link.metadata().definition() == Some(definition)
            })
            .map(crate::Link::id)
        {
            return existing;
        }

        network.insert_link(
            [profile],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_term(term)
                .with_language(&self.language)
                .with_definition(definition),
        )
    }
}

/// Links inserted when a language profile is declared in a network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageProfileLinks {
    profile: LinkId,
    capabilities: Vec<LinkId>,
}

impl LanguageProfileLinks {
    /// Root profile link.
    #[must_use]
    pub const fn profile(&self) -> LinkId {
        self.profile
    }

    /// Capability child links.
    #[must_use]
    pub fn capabilities(&self) -> &[LinkId] {
        &self.capabilities
    }
}

/// A profile validation failure that can be recorded as a diagnostic link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageProfileViolation {
    feature: String,
    message: String,
}

impl LanguageProfileViolation {
    fn new(feature: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            feature: feature.into(),
            message: message.into(),
        }
    }

    /// Unsupported feature that caused the violation.
    #[must_use]
    pub fn feature(&self) -> &str {
        &self.feature
    }
}

impl fmt::Display for LanguageProfileViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} Unsupported feature: {}.",
            self.message, self.feature
        )
    }
}

impl Error for LanguageProfileViolation {}

fn is_profile_control_term(term: &str) -> bool {
    term.starts_with("language-profile")
        || term.starts_with("translation-rule:")
        || term == "translation-rule"
        || term == "translation-rule-set"
}
