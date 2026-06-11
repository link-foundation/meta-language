use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::link_network::{Link, LinkId, LinkNetwork, LinkType};
use crate::query::{LinkQuery, QueryCaptures, QueryMatch, QueryPredicate, QueryPredicateHost};
use crate::source::{ByteRange, SourceSpan};
use crate::substitution::{SubstitutionReport, SubstitutionRule, VariableSubstitutionRule};

/// Replacement rule used by the query-and-transform surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplacementRule {
    kind: ReplacementKind,
}

impl ReplacementRule {
    /// Replaces the source text covered by links captured under `capture_name`.
    ///
    /// Captured syntax links are rewritten by changing the token links inside
    /// the captured range, so all tokens outside the captured links keep their
    /// original text and order.
    #[must_use]
    pub fn captured_text(capture_name: impl Into<String>, replacement: impl Into<String>) -> Self {
        Self {
            kind: ReplacementKind::CapturedText {
                capture_name: normalize_capture_name(capture_name),
                replacement: replacement.into(),
            },
        }
    }

    /// Applies an exact-reference substitution via [`LinkNetwork::apply_substitution`].
    #[must_use]
    pub const fn substitution(rule: SubstitutionRule) -> Self {
        Self {
            kind: ReplacementKind::Substitution(rule),
        }
    }

    /// Applies a variable substitution via [`LinkNetwork::apply_variable_substitution`].
    #[must_use]
    pub const fn variable_substitution(rule: VariableSubstitutionRule) -> Self {
        Self {
            kind: ReplacementKind::VariableSubstitution(rule),
        }
    }

    /// Replaces captured source text with a quasiquote template.
    ///
    /// Placeholders use `{{capture_name}}` and are resolved from the same query
    /// match before each replacement is applied.
    #[must_use]
    pub fn quasiquote(capture_name: impl Into<String>, template: QuasiquoteTemplate) -> Self {
        Self {
            kind: ReplacementKind::Quasiquote {
                capture_name: normalize_capture_name(capture_name),
                template,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ReplacementKind {
    CapturedText {
        capture_name: String,
        replacement: String,
    },
    Quasiquote {
        capture_name: String,
        template: QuasiquoteTemplate,
    },
    Substitution(SubstitutionRule),
    VariableSubstitution(VariableSubstitutionRule),
}

/// Result of replacing query-selected links.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplacementReport {
    text_replacements: Vec<TextReplacement>,
    template_errors: Vec<QuasiquoteError>,
    substitution: SubstitutionReport,
}

impl ReplacementReport {
    /// Source-text replacements made for captured links.
    #[must_use]
    pub fn text_replacements(&self) -> &[TextReplacement] {
        &self.text_replacements
    }

    /// Template rendering errors that prevented replacements.
    #[must_use]
    pub fn template_errors(&self) -> &[QuasiquoteError] {
        &self.template_errors
    }

    /// Structural substitution result, when the rule delegates to substitution.
    #[must_use]
    pub const fn substitution(&self) -> &SubstitutionReport {
        &self.substitution
    }

    /// Returns whether the replacement made no text or structural changes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text_replacements.is_empty()
            && self.template_errors.is_empty()
            && self.substitution.created().is_empty()
            && self.substitution.updated().is_empty()
            && self.substitution.deleted().is_empty()
    }
}

/// One source-text replacement applied to captured token links.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextReplacement {
    capture_name: String,
    link_id: LinkId,
    token_ids: Vec<LinkId>,
    span: Option<SourceSpan>,
    old_text: String,
    new_text: String,
}

impl TextReplacement {
    fn new(
        capture_name: &str,
        link_id: LinkId,
        token_ids: Vec<LinkId>,
        span: Option<SourceSpan>,
        old_text: String,
        new_text: &str,
    ) -> Self {
        Self {
            capture_name: capture_name.to_string(),
            link_id,
            token_ids,
            span,
            old_text,
            new_text: new_text.to_string(),
        }
    }

    /// Capture name that produced this replacement.
    #[must_use]
    pub fn capture_name(&self) -> &str {
        &self.capture_name
    }

    /// Captured link whose source text was replaced.
    #[must_use]
    pub const fn link_id(&self) -> LinkId {
        self.link_id
    }

    /// Token links edited to perform the replacement.
    #[must_use]
    pub fn token_ids(&self) -> &[LinkId] {
        &self.token_ids
    }

    /// Source span covered by the edited tokens.
    #[must_use]
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }

    /// Source text reconstructed from the captured tokens before replacement.
    #[must_use]
    pub fn old_text(&self) -> &str {
        &self.old_text
    }

    /// Replacement text written into the captured range.
    #[must_use]
    pub fn new_text(&self) -> &str {
        &self.new_text
    }
}

/// Built-in predicate host for text predicates over query captures.
#[derive(Clone, Copy, Debug, Default)]
pub struct SourceTextPredicateHost;

impl QueryPredicateHost for SourceTextPredicateHost {
    fn evaluate(
        &self,
        predicate: &QueryPredicate,
        captures: &QueryCaptures,
        network: &LinkNetwork,
    ) -> bool {
        let Some((capture_name, literal)) = capture_literal_arguments(predicate) else {
            return false;
        };
        let Some(captured_text) = captured_text(network, captures.first(capture_name)) else {
            return false;
        };

        match predicate.name() {
            "eq?" => captured_text == literal,
            "not-eq?" => captured_text != literal,
            _ => false,
        }
    }
}

/// Quasiquote replacement template with `{{capture}}` placeholders.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuasiquoteTemplate {
    parts: Vec<TemplatePart>,
}

impl QuasiquoteTemplate {
    /// Parses a template source string.
    pub fn parse(source: impl Into<String>) -> Result<Self, QuasiquoteError> {
        let source = source.into();
        let mut parts = Vec::new();
        let mut rest = source.as_str();
        while let Some(start) = rest.find("{{") {
            if start > 0 {
                parts.push(TemplatePart::Literal(rest[..start].to_string()));
            }
            let after_open = &rest[start + 2..];
            let Some(end) = after_open.find("}}") else {
                return Err(QuasiquoteError::Parse(
                    "unterminated quasiquote placeholder".to_string(),
                ));
            };
            let name = normalize_capture_name(after_open[..end].trim());
            if name.is_empty() {
                return Err(QuasiquoteError::Parse(
                    "quasiquote placeholder is empty".to_string(),
                ));
            }
            parts.push(TemplatePart::Placeholder(name));
            rest = &after_open[end + 2..];
        }
        if !rest.is_empty() {
            parts.push(TemplatePart::Literal(rest.to_string()));
        }
        if parts.is_empty() {
            parts.push(TemplatePart::Literal(source));
        }
        Ok(Self { parts })
    }

    fn render(
        &self,
        network: &LinkNetwork,
        query_match: &QueryMatch,
        old_text: &str,
    ) -> Result<String, QuasiquoteError> {
        let mut values = BTreeMap::<String, String>::new();
        for part in &self.parts {
            if let TemplatePart::Placeholder(name) = part {
                if values.contains_key(name) {
                    continue;
                }
                let Some(text) = captured_text(network, query_match.captures().first(name)) else {
                    return Err(QuasiquoteError::MissingPlaceholder(name.clone()));
                };
                values.insert(name.clone(), text);
            }
        }

        let mut rendered = String::new();
        for part in &self.parts {
            match part {
                TemplatePart::Literal(literal) => rendered.push_str(literal),
                TemplatePart::Placeholder(name) => {
                    let Some(value) = values.get(name) else {
                        return Err(QuasiquoteError::MissingPlaceholder(name.clone()));
                    };
                    rendered.push_str(value);
                }
            }
        }
        Ok(preserve_parentheses(old_text, rendered))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TemplatePart {
    Literal(String),
    Placeholder(String),
}

/// Error returned while parsing or rendering a quasiquote template.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuasiquoteError {
    /// Template source is malformed.
    Parse(String),
    /// Template references a capture that is not bound by the query match.
    MissingPlaceholder(String),
}

impl fmt::Display for QuasiquoteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(message) => formatter.write_str(message),
            Self::MissingPlaceholder(name) => {
                write!(formatter, "quasiquote placeholder `{name}` is not captured")
            }
        }
    }
}

impl std::error::Error for QuasiquoteError {}

impl LinkNetwork {
    /// Finds query matches using the transform surface's source-text predicates.
    ///
    /// This delegates structural matching to [`LinkQuery`]'s S-expression
    /// matcher. Built-in predicates such as `#eq? @capture "text"` compare the
    /// text reconstructed from captured token links.
    #[must_use]
    pub fn find(&self, query: &LinkQuery) -> Vec<QueryMatch> {
        self.query_matches_with(query, &SourceTextPredicateHost)
    }

    /// Applies a replacement rule to links selected by [`LinkNetwork::find`].
    pub fn replace(&mut self, matches: &[QueryMatch], rule: &ReplacementRule) -> ReplacementReport {
        match &rule.kind {
            ReplacementKind::CapturedText {
                capture_name,
                replacement,
            } => ReplacementReport {
                text_replacements: self.replace_captured_text(matches, capture_name, replacement),
                template_errors: Vec::new(),
                substitution: SubstitutionReport::default(),
            },
            ReplacementKind::Quasiquote {
                capture_name,
                template,
            } => {
                let (text_replacements, template_errors) =
                    self.replace_captured_quasiquote(matches, capture_name, template);
                ReplacementReport {
                    text_replacements,
                    template_errors,
                    substitution: SubstitutionReport::default(),
                }
            }
            ReplacementKind::Substitution(rule) => {
                if matches.is_empty() {
                    ReplacementReport::default()
                } else {
                    ReplacementReport {
                        text_replacements: Vec::new(),
                        template_errors: Vec::new(),
                        substitution: self.apply_substitution(rule),
                    }
                }
            }
            ReplacementKind::VariableSubstitution(rule) => {
                if matches.is_empty() {
                    ReplacementReport::default()
                } else {
                    ReplacementReport {
                        text_replacements: Vec::new(),
                        template_errors: Vec::new(),
                        substitution: self.apply_variable_substitution(rule),
                    }
                }
            }
        }
    }

    fn replace_captured_text(
        &mut self,
        matches: &[QueryMatch],
        capture_name: &str,
        replacement: &str,
    ) -> Vec<TextReplacement> {
        let mut touched_tokens = BTreeSet::new();
        let mut replacements = Vec::new();

        for query_match in matches {
            for capture in query_match
                .captures()
                .iter()
                .filter(|capture| capture.name() == capture_name)
            {
                let token_ids = source_token_ids(self, capture.link_id());
                if token_ids.is_empty()
                    || token_ids
                        .iter()
                        .any(|token_id| touched_tokens.contains(token_id))
                {
                    continue;
                }

                let old_text = text_for_tokens(self, &token_ids);
                if old_text == replacement {
                    continue;
                }

                let span = span_for_tokens(self, &token_ids);
                let first_token = token_ids[0];
                if !self.set_term(first_token, replacement.to_string()) {
                    continue;
                }
                for token_id in token_ids.iter().skip(1) {
                    let _ = self.set_term(*token_id, String::new());
                }

                touched_tokens.extend(token_ids.iter().copied());
                replacements.push(TextReplacement::new(
                    capture_name,
                    capture.link_id(),
                    token_ids,
                    span,
                    old_text,
                    replacement,
                ));
            }
        }

        replacements
    }

    fn replace_captured_quasiquote(
        &mut self,
        matches: &[QueryMatch],
        capture_name: &str,
        template: &QuasiquoteTemplate,
    ) -> (Vec<TextReplacement>, Vec<QuasiquoteError>) {
        let mut touched_tokens = BTreeSet::new();
        let mut replacements = Vec::new();
        let mut errors = Vec::new();

        for query_match in matches {
            for capture in query_match
                .captures()
                .iter()
                .filter(|capture| capture.name() == capture_name)
            {
                let token_ids = source_token_ids(self, capture.link_id());
                if token_ids.is_empty()
                    || token_ids
                        .iter()
                        .any(|token_id| touched_tokens.contains(token_id))
                {
                    continue;
                }

                let old_text = text_for_tokens(self, &token_ids);
                let replacement = match template.render(self, query_match, &old_text) {
                    Ok(replacement) => replacement,
                    Err(error) => {
                        errors.push(error);
                        continue;
                    }
                };
                if old_text == replacement {
                    continue;
                }

                let span = span_for_tokens(self, &token_ids);
                let first_token = token_ids[0];
                if !self.set_term(first_token, replacement.clone()) {
                    continue;
                }
                for token_id in token_ids.iter().skip(1) {
                    let _ = self.set_term(*token_id, String::new());
                }

                touched_tokens.extend(token_ids.iter().copied());
                replacements.push(TextReplacement::new(
                    capture_name,
                    capture.link_id(),
                    token_ids,
                    span,
                    old_text,
                    &replacement,
                ));
            }
        }

        (replacements, errors)
    }
}

fn normalize_capture_name(name: impl Into<String>) -> String {
    name.into().trim_start_matches('@').to_string()
}

fn preserve_parentheses(old_text: &str, rendered: String) -> String {
    let trimmed_old = old_text.trim();
    let trimmed_rendered = rendered.trim();
    if trimmed_old.starts_with('(')
        && trimmed_old.ends_with(')')
        && !(trimmed_rendered.starts_with('(') && trimmed_rendered.ends_with(')'))
    {
        format!("({rendered})")
    } else {
        rendered
    }
}

fn capture_literal_arguments(predicate: &QueryPredicate) -> Option<(&str, &str)> {
    let [capture_argument, literal_argument] = predicate.arguments() else {
        return None;
    };
    Some((
        capture_argument.capture_name()?,
        literal_argument.literal()?,
    ))
}

fn captured_text(network: &LinkNetwork, link_id: Option<LinkId>) -> Option<String> {
    let link_id = link_id?;
    let token_ids = source_token_ids(network, link_id);
    if token_ids.is_empty() {
        network
            .link(link_id)
            .and_then(|link| link.metadata().term())
            .map(str::to_string)
    } else {
        Some(text_for_tokens(network, &token_ids))
    }
}

fn source_token_ids(network: &LinkNetwork, link_id: LinkId) -> Vec<LinkId> {
    let mut visited = BTreeSet::new();
    let mut token_ids = Vec::new();
    collect_source_tokens(network, link_id, &mut visited, &mut token_ids);
    token_ids.sort_by_key(|token_id| token_sort_key(network, *token_id));
    token_ids.dedup();
    token_ids
}

fn collect_source_tokens(
    network: &LinkNetwork,
    link_id: LinkId,
    visited: &mut BTreeSet<LinkId>,
    token_ids: &mut Vec<LinkId>,
) {
    if !visited.insert(link_id) {
        return;
    }
    let Some(link) = network.link(link_id) else {
        return;
    };

    match link.metadata().link_type() {
        Some(LinkType::Token) => {
            if !link.metadata().flags().is_missing() {
                token_ids.push(link_id);
            }
            return;
        }
        Some(LinkType::Field | LinkType::Trivia) => return,
        _ => {}
    }

    let children = network
        .links()
        .filter(|candidate| candidate.references().first().copied() == Some(link_id))
        .map(Link::id)
        .collect::<Vec<_>>();
    for child in children {
        collect_source_tokens(network, child, visited, token_ids);
    }
}

fn token_sort_key(network: &LinkNetwork, token_id: LinkId) -> (usize, u64) {
    let start = network
        .link(token_id)
        .and_then(|link| link.metadata().span())
        .map_or(usize::MAX, |span| span.byte_range().start());
    (start, token_id.as_u64())
}

fn text_for_tokens(network: &LinkNetwork, token_ids: &[LinkId]) -> String {
    token_ids
        .iter()
        .filter_map(|token_id| network.link(*token_id))
        .filter_map(|link| link.metadata().term())
        .collect()
}

fn span_for_tokens(network: &LinkNetwork, token_ids: &[LinkId]) -> Option<SourceSpan> {
    let spans = token_ids
        .iter()
        .filter_map(|token_id| network.link(*token_id))
        .filter_map(|link| link.metadata().span())
        .collect::<Vec<_>>();
    let first = spans.first()?;
    let last = spans.last()?;
    Some(SourceSpan::new(
        ByteRange::new(first.byte_range().start(), last.byte_range().end()),
        first.start_point(),
        last.end_point(),
    ))
}
