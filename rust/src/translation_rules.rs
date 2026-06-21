use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use crate::{
    FormalizationLevel, Link, LinkId, LinkMetadata, LinkNetwork, LinkQuery, LinkType,
    LinoSerializationError, NaturalizationDirection, ParseConfiguration, QueryMatch,
    QueryParseError,
};

const RULE_SET_TERM: &str = "translation-rule-set";
const RULE_TERM: &str = "translation-rule";
const MATCH_TERM: &str = "translation-rule-match";
const REFERENCE_CAPTURE_LANGUAGE: &str = "translation-rule-reference-capture";
const TEMPLATE_DEFINITION: &str = "translation-rule-template";
const FORMAL_LEXICAL_TARGET: &str = "formal:lexical";
const FORMAL_CONCEPT_TARGET: &str = "formal:concept";
const FORMAL_LOGICAL_TARGET: &str = "formal:logical";

/// Ordered collection of named translation rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationRuleSet {
    name: String,
    rules: Vec<TranslationRule>,
}

impl TranslationRuleSet {
    /// Creates an empty named rule set.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rules: Vec::new(),
        }
    }

    /// Human-readable rule-set name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Ordered rules in this set.
    #[must_use]
    pub fn rules(&self) -> &[TranslationRule] {
        &self.rules
    }

    /// Returns a copy with one rule appended.
    #[must_use]
    pub fn with_rule(mut self, rule: TranslationRule) -> Self {
        self.add_rule(rule);
        self
    }

    /// Appends a rule to the end of the ordered rule set.
    pub fn add_rule(&mut self, rule: TranslationRule) {
        self.rules.push(rule);
    }

    /// Serializes this rule set through the existing canonical `LiNo` network format.
    #[must_use]
    pub fn to_lino(&self) -> String {
        let mut network = LinkNetwork::new();
        let root = network.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_term(RULE_SET_TERM)
                .with_definition(&self.name),
        );

        for rule in &self.rules {
            let rule_link = network.insert_link(
                [root],
                LinkMetadata::new()
                    .with_link_type(LinkType::Semantic)
                    .with_named(true)
                    .with_term(RULE_TERM)
                    .with_definition(rule.name()),
            );
            network.insert_link(
                [rule_link],
                LinkMetadata::new()
                    .with_link_type(LinkType::Semantic)
                    .with_named(true)
                    .with_term(MATCH_TERM)
                    .with_definition(query_to_rule_spec(&rule.query)),
            );
            for (capture, reference_index) in &rule.reference_captures {
                network.insert_link(
                    [rule_link],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Semantic)
                        .with_named(true)
                        .with_term(capture)
                        .with_language(REFERENCE_CAPTURE_LANGUAGE)
                        .with_definition(reference_index.to_string()),
                );
            }
            for (target, template) in &rule.templates {
                network.insert_link(
                    [rule_link],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Semantic)
                        .with_named(true)
                        .with_term(template.source())
                        .with_language(target)
                        .with_definition(TEMPLATE_DEFINITION),
                );
            }
        }

        network.to_lino()
    }

    /// Loads a rule set from `LiNo` text produced by [`TranslationRuleSet::to_lino`].
    pub fn from_lino(text: &str) -> Result<Self, TranslationRuleSetLoadError> {
        let network = LinkNetwork::from_lino(text)?;
        let root = network
            .links()
            .find(|link| {
                link.metadata().link_type() == Some(LinkType::Semantic)
                    && link.metadata().term() == Some(RULE_SET_TERM)
            })
            .ok_or_else(|| {
                TranslationRuleSetLoadError::Structure(
                    "missing translation-rule-set root".to_string(),
                )
            })?;
        let mut rules = Vec::new();
        let mut rule_links = network
            .links()
            .filter(|link| {
                link.references().first().copied() == Some(root.id())
                    && link.metadata().term() == Some(RULE_TERM)
            })
            .collect::<Vec<_>>();
        rule_links.sort_by_key(|link| link.id());

        for rule_link in rule_links {
            rules.push(load_rule(&network, rule_link)?);
        }

        Ok(Self {
            name: root
                .metadata()
                .definition()
                .unwrap_or(RULE_SET_TERM)
                .to_string(),
            rules,
        })
    }

    /// `LiNo` text for the built-in statehood demo rule set.
    #[must_use]
    pub fn statehood_demo_lino() -> &'static str {
        static LINO: OnceLock<String> = OnceLock::new();
        LINO.get_or_init(|| statehood_demo_rule_set().to_lino())
    }

    /// Loads the statehood demo from its `LiNo` rule-set representation.
    #[must_use]
    pub fn statehood_demo() -> Self {
        Self::from_lino(Self::statehood_demo_lino())
            .expect("statehood demo translation rule set must load")
    }

    pub(crate) fn render(
        &self,
        network: &LinkNetwork,
        target_language: &str,
        configuration: ParseConfiguration,
    ) -> Option<String> {
        let source = network.reconstruct_text();
        for rule in &self.rules {
            let Some(template) = rule.template_for(target_language, configuration) else {
                continue;
            };
            let rendered = network
                .query_matches(&rule.query)
                .into_iter()
                .map(|query_match| template.render(network, rule, &query_match, target_language))
                .collect::<Vec<_>>();

            if !rendered.is_empty() {
                return Some(with_source_trailing_newline(rendered.join("\n"), &source));
            }
        }

        None
    }
}

/// A named translation rule with one match query and target-language templates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationRule {
    name: String,
    query: LinkQuery,
    reference_captures: BTreeMap<String, usize>,
    templates: BTreeMap<String, TranslationTemplate>,
}

impl TranslationRule {
    /// Creates a named rule that matches links selected by `query`.
    #[must_use]
    pub fn new(name: impl Into<String>, query: LinkQuery) -> Self {
        Self {
            name: name.into(),
            query,
            reference_captures: BTreeMap::new(),
            templates: BTreeMap::new(),
        }
    }

    /// Rule name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Link query used by the rule.
    #[must_use]
    pub const fn query(&self) -> &LinkQuery {
        &self.query
    }

    /// Reference-index captures available to templates.
    #[must_use]
    pub const fn reference_captures(&self) -> &BTreeMap<String, usize> {
        &self.reference_captures
    }

    /// Target templates keyed by language or formal target.
    #[must_use]
    pub const fn templates(&self) -> &BTreeMap<String, TranslationTemplate> {
        &self.templates
    }

    /// Adds a placeholder binding that captures one reference from the matched link.
    #[must_use]
    pub fn with_reference_capture(
        mut self,
        name: impl Into<String>,
        reference_index: usize,
    ) -> Self {
        self.reference_captures.insert(name.into(), reference_index);
        self
    }

    /// Adds a template for a natural language target such as `English` or `Russian`.
    #[must_use]
    pub fn with_template(
        mut self,
        target_language: impl Into<String>,
        template: impl Into<String>,
    ) -> Self {
        self.templates.insert(
            target_language.into(),
            TranslationTemplate::new(template.into()),
        );
        self
    }

    /// Adds a template for a formalization level.
    #[must_use]
    pub fn with_formal_template(
        mut self,
        level: FormalizationLevel,
        template: impl Into<String>,
    ) -> Self {
        self.templates.insert(
            formal_template_target(level).to_string(),
            TranslationTemplate::new(template.into()),
        );
        self
    }

    fn template_for(
        &self,
        target_language: &str,
        configuration: ParseConfiguration,
    ) -> Option<&TranslationTemplate> {
        let level = effective_formalization_level(configuration);
        if level != FormalizationLevel::Natural {
            return self.templates.get(formal_template_target(level));
        }

        self.templates.get(target_language).or_else(|| {
            canonical_reconstruction_language(target_language)
                .and_then(|language| self.templates.get(language))
        })
    }
}

/// A quasiquote-style target template with `{placeholder}` substitutions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationTemplate {
    source: String,
}

impl TranslationTemplate {
    /// Creates a template from source text.
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }

    /// Template source text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    fn render(
        &self,
        network: &LinkNetwork,
        rule: &TranslationRule,
        query_match: &QueryMatch,
        target_language: &str,
    ) -> String {
        let mut output = String::new();
        let mut chars = self.source.chars().peekable();
        while let Some(character) = chars.next() {
            match character {
                '{' if chars.peek() == Some(&'{') => {
                    chars.next();
                    output.push('{');
                }
                '{' => {
                    let mut placeholder = String::new();
                    let mut closed = false;
                    for next in chars.by_ref() {
                        if next == '}' {
                            closed = true;
                            break;
                        }
                        placeholder.push(next);
                    }
                    if closed {
                        output.push_str(&render_placeholder(
                            network,
                            rule,
                            query_match,
                            target_language,
                            &placeholder,
                        ));
                    } else {
                        output.push('{');
                        output.push_str(&placeholder);
                    }
                }
                '}' if chars.peek() == Some(&'}') => {
                    chars.next();
                    output.push('}');
                }
                other => output.push(other),
            }
        }
        output
    }
}

/// Runtime registry for selecting and replacing active translation rule sets.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TranslationRuleRegistry {
    rule_sets: BTreeMap<String, TranslationRuleSet>,
    active_rule_set: Option<String>,
}

impl TranslationRuleRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry containing the statehood demo rule set.
    #[must_use]
    pub fn with_statehood_demo() -> Self {
        Self::new().with_rule_set(TranslationRuleSet::statehood_demo())
    }

    /// Returns a copy with a rule set registered.
    #[must_use]
    pub fn with_rule_set(mut self, rule_set: TranslationRuleSet) -> Self {
        self.replace_rule_set(rule_set);
        self
    }

    /// Inserts or replaces a rule set. The first registered set becomes active.
    pub fn replace_rule_set(&mut self, rule_set: TranslationRuleSet) {
        let name = rule_set.name().to_string();
        if self.active_rule_set.is_none() {
            self.active_rule_set = Some(name.clone());
        }
        self.rule_sets.insert(name, rule_set);
    }

    /// Selects the active rule set by name.
    pub fn set_active_rule_set(&mut self, name: &str) -> bool {
        if self.rule_sets.contains_key(name) {
            self.active_rule_set = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Returns the active rule set.
    #[must_use]
    pub fn active_rule_set(&self) -> Option<&TranslationRuleSet> {
        self.active_rule_set
            .as_deref()
            .and_then(|name| self.rule_sets.get(name))
    }

    /// Looks up a rule set by name.
    #[must_use]
    pub fn rule_set(&self, name: &str) -> Option<&TranslationRuleSet> {
        self.rule_sets.get(name)
    }

    /// Number of registered rule sets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rule_sets.len()
    }

    /// Whether the registry has no rule sets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rule_sets.is_empty()
    }
}

/// Error returned when a rule set cannot be loaded from `LiNo`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationRuleSetLoadError {
    /// The underlying network `LiNo` failed to load.
    Lino(LinoSerializationError),
    /// The loaded network does not match the translation rule schema.
    Structure(String),
    /// A persisted `LinkQuery` failed to parse.
    Query(QueryParseError),
}

impl fmt::Display for TranslationRuleSetLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lino(error) => write!(formatter, "{error}"),
            Self::Structure(message) => {
                write!(formatter, "translation rule structure error: {message}")
            }
            Self::Query(error) => write!(formatter, "translation rule query error: {error}"),
        }
    }
}

impl Error for TranslationRuleSetLoadError {}

impl From<LinoSerializationError> for TranslationRuleSetLoadError {
    fn from(error: LinoSerializationError) -> Self {
        Self::Lino(error)
    }
}

impl From<QueryParseError> for TranslationRuleSetLoadError {
    fn from(error: QueryParseError) -> Self {
        Self::Query(error)
    }
}

fn load_rule(
    network: &LinkNetwork,
    rule_link: &Link,
) -> Result<TranslationRule, TranslationRuleSetLoadError> {
    let name = rule_link.metadata().definition().ok_or_else(|| {
        TranslationRuleSetLoadError::Structure("rule is missing a name".to_string())
    })?;
    let query_source = network
        .links()
        .find(|link| {
            link.references().first().copied() == Some(rule_link.id())
                && link.metadata().term() == Some(MATCH_TERM)
        })
        .and_then(|link| link.metadata().definition())
        .ok_or_else(|| {
            TranslationRuleSetLoadError::Structure("rule is missing a match query".to_string())
        })?;
    let mut rule = TranslationRule::new(name, query_from_rule_spec(query_source)?);
    let mut children = network
        .links()
        .filter(|link| link.references().first().copied() == Some(rule_link.id()))
        .collect::<Vec<_>>();
    children.sort_by_key(|link| link.id());

    for child in children {
        let metadata = child.metadata();
        if metadata.term() == Some(MATCH_TERM) {
            continue;
        }
        if metadata.language() == Some(REFERENCE_CAPTURE_LANGUAGE) {
            let capture = metadata.term().ok_or_else(|| {
                TranslationRuleSetLoadError::Structure(
                    "reference capture is missing a capture name".to_string(),
                )
            })?;
            let index = metadata
                .definition()
                .ok_or_else(|| {
                    TranslationRuleSetLoadError::Structure(
                        "reference capture is missing an index".to_string(),
                    )
                })?
                .parse::<usize>()
                .map_err(|error| {
                    TranslationRuleSetLoadError::Structure(format!(
                        "invalid reference capture index: {error}"
                    ))
                })?;
            rule = rule.with_reference_capture(capture, index);
        } else if metadata.definition() == Some(TEMPLATE_DEFINITION) {
            let target = metadata.language().ok_or_else(|| {
                TranslationRuleSetLoadError::Structure("template is missing a target".to_string())
            })?;
            let template = metadata.term().ok_or_else(|| {
                TranslationRuleSetLoadError::Structure(
                    "template is missing source text".to_string(),
                )
            })?;
            rule = rule.with_template(target, template);
        }
    }

    Ok(rule)
}

fn query_to_rule_spec(query: &LinkQuery) -> String {
    let mut object = serde_json::Map::new();
    if let Some(link_type) = query.link_type_filter() {
        object.insert("link_type".to_string(), link_type.to_string().into());
    }
    if let Some(term) = query.term_filter() {
        object.insert("term".to_string(), term.into());
    }
    if let Some(language) = query.language_filter() {
        object.insert("language".to_string(), language.into());
    }
    if let Some(named) = query.named_filter() {
        object.insert("named".to_string(), named.into());
    }
    if let Some(pattern_source) = query.pattern_source() {
        object.insert("sexpression".to_string(), pattern_source.into());
    }

    serde_json::Value::Object(object).to_string()
}

fn query_from_rule_spec(source: &str) -> Result<LinkQuery, QueryParseError> {
    let value = serde_json::from_str::<serde_json::Value>(source)
        .map_err(|error| QueryParseError::new(format!("invalid query spec: {error}")))?;
    let object = value
        .as_object()
        .ok_or_else(|| QueryParseError::new("query spec must be a JSON object"))?;

    let mut query =
        if let Some(sexpression) = object.get("sexpression").and_then(|value| value.as_str()) {
            LinkQuery::from_sexpression(sexpression)?
        } else {
            LinkQuery::new()
        };

    if let Some(link_type) = object.get("link_type").and_then(|value| value.as_str()) {
        query = query.with_link_type(parse_query_link_type(link_type)?);
    }
    if let Some(term) = object.get("term").and_then(|value| value.as_str()) {
        query = query.with_term(term);
    }
    if let Some(language) = object.get("language").and_then(|value| value.as_str()) {
        query = query.with_language(language);
    }
    if let Some(named) = object.get("named").and_then(serde_json::Value::as_bool) {
        query = query.with_named(named);
    }

    Ok(query)
}

fn parse_query_link_type(token: &str) -> Result<LinkType, QueryParseError> {
    Ok(match token {
        "link" => LinkType::Link,
        "reference" => LinkType::Reference,
        "relation" => LinkType::Relation,
        "language" => LinkType::Language,
        "grammar" => LinkType::Grammar,
        "type" => LinkType::Type,
        "concept" => LinkType::Concept,
        "syntax" => LinkType::Syntax,
        "field" => LinkType::Field,
        "trivia" => LinkType::Trivia,
        "token" => LinkType::Token,
        "document" => LinkType::Document,
        "semantic" => LinkType::Semantic,
        "region" => LinkType::Region,
        "object" => LinkType::Object,
        other => {
            return Err(QueryParseError::new(format!(
                "unknown query link type `{other}`"
            )))
        }
    })
}

fn render_placeholder(
    network: &LinkNetwork,
    rule: &TranslationRule,
    query_match: &QueryMatch,
    target_language: &str,
    placeholder: &str,
) -> String {
    let (name, mode) = placeholder.split_once(':').map_or_else(
        || (placeholder.trim(), "language"),
        |(name, mode)| (name.trim(), mode.trim()),
    );
    let Some(link_id) = placeholder_link(network, rule, query_match, name) else {
        return format!("{{{placeholder}}}");
    };

    render_link(network, link_id, target_language, mode)
}

fn placeholder_link(
    network: &LinkNetwork,
    rule: &TranslationRule,
    query_match: &QueryMatch,
    name: &str,
) -> Option<LinkId> {
    if let Some(link_id) = query_match.captures().first(name) {
        return Some(link_id);
    }

    let reference_index = *rule.reference_captures.get(name)?;
    network
        .link(query_match.link_id())?
        .references()
        .get(reference_index)
        .copied()
}

fn render_link(
    network: &LinkNetwork,
    link_id: LinkId,
    target_language: &str,
    mode: &str,
) -> String {
    let Some(link) = network.link(link_id) else {
        return link_id.to_string();
    };
    let concept = concept_id_for_link(network, link);
    match mode {
        "concept" => concept
            .or_else(|| link.metadata().term())
            .map_or_else(|| link_id.to_string(), str::to_string),
        "term" => link
            .metadata()
            .term()
            .map_or_else(|| link_id.to_string(), str::to_string),
        _ => concept
            .and_then(|concept| reconstruct_concept_for_language(network, concept, target_language))
            .or_else(|| link.metadata().term())
            .map_or_else(|| link_id.to_string(), str::to_string),
    }
}

fn concept_id_for_link<'a>(network: &'a LinkNetwork, link: &'a Link) -> Option<&'a str> {
    if link.metadata().link_type() == Some(LinkType::Concept) {
        return link.metadata().term();
    }
    let first_reference = link.references().first().copied()?;
    let concept = network.link(first_reference)?;
    (concept.metadata().link_type() == Some(LinkType::Concept))
        .then(|| concept.metadata().term())
        .flatten()
}

fn reconstruct_concept_for_language<'a>(
    network: &'a LinkNetwork,
    concept: &str,
    language: &str,
) -> Option<&'a str> {
    network.reconstruct_concept(concept, language).or_else(|| {
        canonical_reconstruction_language(language)
            .and_then(|canonical| network.reconstruct_concept(concept, canonical))
    })
}

fn statehood_demo_rule_set() -> TranslationRuleSet {
    TranslationRuleSet::new("statehood-demo").with_rule(
        TranslationRule::new(
            "statehood proposition",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:statehood"),
        )
        .with_reference_capture("subject", 2)
        .with_reference_capture("object", 3)
        .with_template("English", "{subject} is a {object}.")
        .with_template("en", "{subject} is a {object}.")
        .with_template("Russian", "{subject} это {object}.")
        .with_template("ru", "{subject} это {object}.")
        .with_formal_template(
            FormalizationLevel::Lexical,
            "statehood({subject}, {object})",
        )
        .with_formal_template(
            FormalizationLevel::Concept,
            [
                "statehood(",
                "{subject:concept}",
                ", ",
                "{object:concept}",
                ")",
            ]
            .concat(),
        )
        .with_formal_template(
            FormalizationLevel::Logical,
            [
                "(proposition: statehood (subject: ",
                "{subject:concept}",
                ") (object: ",
                "{object:concept}",
                ") (truth: true))",
            ]
            .concat(),
        ),
    )
}

const fn effective_formalization_level(configuration: ParseConfiguration) -> FormalizationLevel {
    match (
        configuration.naturalization_direction(),
        configuration.formalization_level(),
    ) {
        (NaturalizationDirection::Formalize, FormalizationLevel::Natural) => {
            FormalizationLevel::Lexical
        }
        (_, level) => level,
    }
}

const fn formal_template_target(level: FormalizationLevel) -> &'static str {
    match level {
        FormalizationLevel::Natural => "",
        FormalizationLevel::Lexical => FORMAL_LEXICAL_TARGET,
        FormalizationLevel::Concept => FORMAL_CONCEPT_TARGET,
        FormalizationLevel::Logical => FORMAL_LOGICAL_TARGET,
    }
}

fn canonical_reconstruction_language(language: &str) -> Option<&'static str> {
    match language.to_ascii_lowercase().as_str() {
        "english" | "en" => Some("English"),
        "russian" | "ru" => Some("Russian"),
        _ => None,
    }
}

fn with_source_trailing_newline(mut body: String, source: &str) -> String {
    if source.ends_with('\n') {
        body.push('\n');
    }
    body
}
