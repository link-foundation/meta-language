use std::collections::BTreeMap;
use std::fmt;

use crate::configuration::{ParseConfiguration, RegionDetectionPolicy, TriviaAttachmentPolicy};
use crate::language_parser::{BuiltInLanguageParser, LanguageParser};
use crate::link_flags::LinkFlags;
use crate::mixed_regions::{detect_embedded_regions, EmbeddedRegion};
use crate::query::LinkQuery;
use crate::source::{ByteRange, Point, SourceSpan};
use crate::substitution::{SubstitutionReport, SubstitutionRule};
use crate::verification::{VerificationIssue, VerificationIssueKind, VerificationReport};

/// Stable identifier for a link inside a [`LinkNetwork`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkId(u64);

impl LinkId {
    /// Returns the numeric identifier.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for LinkId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Coarse role for a link in the meta-language network.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkType {
    Link,
    Reference,
    Relation,
    Language,
    Grammar,
    Type,
    Concept,
    Syntax,
    Field,
    Trivia,
    Token,
    Document,
    Semantic,
    Region,
    Object,
}

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

    fn includes(self, link: &Link) -> bool {
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

impl fmt::Display for LinkType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Link => "link",
            Self::Reference => "reference",
            Self::Relation => "relation",
            Self::Language => "language",
            Self::Grammar => "grammar",
            Self::Type => "type",
            Self::Concept => "concept",
            Self::Syntax => "syntax",
            Self::Field => "field",
            Self::Trivia => "trivia",
            Self::Token => "token",
            Self::Document => "document",
            Self::Semantic => "semantic",
            Self::Region => "region",
            Self::Object => "object",
        };
        formatter.write_str(name)
    }
}

/// Metadata carried by a link.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkMetadata {
    link_type: Option<LinkType>,
    named: bool,
    term: Option<String>,
    definition: Option<String>,
    language: Option<String>,
    span: Option<SourceSpan>,
    flags: LinkFlags,
}

impl LinkMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns metadata with a link type.
    #[must_use]
    pub const fn with_link_type(mut self, link_type: LinkType) -> Self {
        self.link_type = Some(link_type);
        self
    }

    /// Returns metadata with the named flag set.
    #[must_use]
    pub const fn with_named(mut self, named: bool) -> Self {
        self.named = named;
        self
    }

    /// Returns metadata with a term label.
    #[must_use]
    pub fn with_term(mut self, term: impl Into<String>) -> Self {
        self.term = Some(term.into());
        self
    }

    /// Returns metadata with a self-description definition.
    #[must_use]
    pub fn with_definition(mut self, definition: impl Into<String>) -> Self {
        self.definition = Some(definition.into());
        self
    }

    /// Returns metadata with a language label.
    #[must_use]
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Returns metadata with a source span.
    #[must_use]
    pub const fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Returns metadata with parse status flags.
    #[must_use]
    pub const fn with_flags(mut self, flags: LinkFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Link type, when known.
    #[must_use]
    pub const fn link_type(&self) -> Option<LinkType> {
        self.link_type
    }

    /// Whether this link is named.
    #[must_use]
    pub const fn is_named(&self) -> bool {
        self.named
    }

    /// Term label attached to this link.
    #[must_use]
    pub fn term(&self) -> Option<&str> {
        self.term.as_deref()
    }

    /// Self-description definition attached to this link.
    #[must_use]
    pub fn definition(&self) -> Option<&str> {
        self.definition.as_deref()
    }

    /// Language label attached to this link.
    #[must_use]
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Source span attached to this link.
    #[must_use]
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }

    /// Parse status flags attached to this link.
    #[must_use]
    pub const fn flags(&self) -> LinkFlags {
        self.flags
    }
}

/// A link is an n-tuple of references to other links.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    id: LinkId,
    references: Vec<LinkId>,
    metadata: LinkMetadata,
}

impl Link {
    /// Link identifier.
    #[must_use]
    pub const fn id(&self) -> LinkId {
        self.id
    }

    /// Ordered references carried by this link.
    #[must_use]
    pub fn references(&self) -> &[LinkId] {
        &self.references
    }

    /// Metadata carried by this link.
    #[must_use]
    pub const fn metadata(&self) -> &LinkMetadata {
        &self.metadata
    }

    const fn metadata_mut(&mut self) -> &mut LinkMetadata {
        &mut self.metadata
    }
}

/// Mutable links network for CST, AST, semantic, and self-description links.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkNetwork {
    next_id: u64,
    links: BTreeMap<LinkId, Link>,
    terms: BTreeMap<String, LinkId>,
    concept_syntax: BTreeMap<(String, String), String>,
}

impl LinkNetwork {
    /// Creates an empty links network.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            next_id: 1,
            links: BTreeMap::new(),
            terms: BTreeMap::new(),
            concept_syntax: BTreeMap::new(),
        }
    }

    /// Creates a links network containing the common self-description roots.
    #[must_use]
    pub fn self_describing() -> Self {
        let mut network = Self::new();
        network.insert_typed_point(
            "link",
            LinkType::Link,
            Some("A link is an n-tuple of references to links."),
        );
        network.insert_typed_point(
            "reference",
            LinkType::Reference,
            Some("A reference is one position in a link that points to another link."),
        );
        network.insert_typed_point(
            "relation link",
            LinkType::Relation,
            Some("A relation link connects references to other links and is itself a link."),
        );
        network.insert_typed_point(
            "language",
            LinkType::Language,
            Some("A language is a set of grammar, syntax, and semantic links."),
        );
        network.insert_typed_point(
            "grammar",
            LinkType::Grammar,
            Some("A grammar describes which syntax links fully match a language."),
        );
        network.insert_typed_point(
            "type",
            LinkType::Type,
            Some("A type is a link that constrains or classifies other links."),
        );
        network.insert_typed_point(
            "concept",
            LinkType::Concept,
            Some("A concept is a shared meaning link that multiple languages can reference."),
        );
        network.insert_typed_point(
            "point",
            LinkType::Concept,
            Some("A point is represented as a self-referential link."),
        );
        network.insert_typed_point(
            "field",
            LinkType::Field,
            Some("A field is a labeled relation link from a parent link to a child link."),
        );
        network.insert_typed_point(
            "trivia",
            LinkType::Trivia,
            Some("Trivia is source text preserved by explicit attachment links."),
        );
        network.insert_typed_point(
            "region",
            LinkType::Region,
            Some("A region is a source span with a selected or detected language."),
        );
        network.insert_typed_point(
            "object",
            LinkType::Object,
            Some("An object identity is represented by a link that other links can share."),
        );
        network
    }

    /// Parses plain source text into a lossless token network.
    ///
    /// This is the default parse operation. It is lossless by construction; use
    /// [`LinkNetwork::projected_links`] when a lower-level view should be
    /// stripped away for CST, AST, or semantic-only work.
    #[must_use]
    pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> Self {
        BuiltInLanguageParser.parse_source(text, language, configuration)
    }

    /// Parses plain source text into a lossless token network.
    ///
    /// This parser boundary preserves source spans, trivia links, recovery
    /// markers, and mixed-region metadata behind the same network
    /// representation.
    #[must_use]
    pub fn parse_lossless_text(
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> Self {
        let (mut network, document) = Self::new_parse_document(text, language);

        let mut row = 0;
        let mut column = 0;
        let mut open_parentheses = Vec::new();
        for (start, character) in text.char_indices() {
            let start_point = Point::new(row, column);
            let end = start + character.len_utf8();
            if character == '\n' {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }
            let end_point = Point::new(row, column);
            let span = SourceSpan::new(ByteRange::new(start, end), start_point, end_point);
            let mut metadata = LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(!character.is_whitespace())
                .with_term(character.to_string())
                .with_language(language)
                .with_span(span);

            if character.is_whitespace() {
                metadata = metadata.with_flags(LinkFlags::extra());
            }

            let token = network.insert_link([document], metadata);
            match character {
                '(' => open_parentheses.push(token),
                ')' if open_parentheses.pop().is_none() => {
                    network.set_flags(token, LinkFlags::error());
                }
                _ => {}
            }
            if character.is_whitespace() {
                network.attach_trivia(
                    document,
                    token,
                    span,
                    configuration.trivia_attachment_policy(),
                );
            }
        }

        let missing_span = SourceSpan::new(
            ByteRange::new(text.len(), text.len()),
            end_point_for_text(text),
            end_point_for_text(text),
        );
        for open_parenthesis in open_parentheses {
            network.set_flags(open_parenthesis, LinkFlags::containing_error());
            network.insert_link(
                [document],
                LinkMetadata::new()
                    .with_link_type(LinkType::Token)
                    .with_named(false)
                    .with_term(")")
                    .with_language(language)
                    .with_span(missing_span)
                    .with_flags(LinkFlags::missing()),
            );
        }

        network.attach_embedded_regions(
            document,
            text,
            language,
            configuration.region_detection_policy(),
        );

        network
    }

    pub(crate) fn new_parse_document(text: &str, language: &str) -> (Self, LinkId) {
        let mut network = Self::self_describing();
        let language_link = network.insert_typed_point(language, LinkType::Language, None);
        let document_span = SourceSpan::new(
            ByteRange::new(0, text.len()),
            Point::new(0, 0),
            end_point_for_text(text),
        );
        let document = network.insert_link(
            [language_link],
            LinkMetadata::new()
                .with_link_type(LinkType::Document)
                .with_named(true)
                .with_term(format!("{language} document"))
                .with_language(language)
                .with_span(document_span),
        );
        (network, document)
    }

    /// Number of links in the network.
    #[must_use]
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Whether the network contains no links.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Iterates over links in identifier order.
    pub fn links(&self) -> impl Iterator<Item = &Link> {
        self.links.values()
    }

    /// Iterates over links included in the selected projection.
    pub fn projected_links(&self, projection: NetworkProjection) -> impl Iterator<Item = &Link> {
        self.links().filter(move |link| projection.includes(link))
    }

    /// Reconstructs source text from non-missing token links ordered by span.
    #[must_use]
    pub fn reconstruct_text(&self) -> String {
        let mut tokens = self
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
            .filter(|link| !link.metadata().flags().is_missing())
            .filter_map(|link| {
                Some((
                    link.metadata().span()?.byte_range().start(),
                    link.id().as_u64(),
                    link.metadata().term()?.to_string(),
                ))
            })
            .collect::<Vec<_>>();

        tokens.sort_by_key(|(start, id, _term)| (*start, *id));
        tokens.into_iter().map(|(_start, _id, term)| term).collect()
    }

    /// Returns embedded mixed-language regions discovered during parse.
    #[must_use]
    pub fn embedded_regions(&self) -> Vec<EmbeddedRegion> {
        self.links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Region))
            .filter_map(|link| {
                Some(EmbeddedRegion::new(
                    link.metadata().language()?.to_string(),
                    link.metadata().span()?,
                ))
            })
            .collect()
    }

    /// Returns links matching a structural query.
    #[must_use]
    pub fn query_links(&self, query: &LinkQuery) -> Vec<&Link> {
        self.links().filter(|link| query.matches(link)).collect()
    }

    /// Inserts a self-referential point link for a term.
    pub fn insert_point(&mut self, term: &str) -> LinkId {
        self.insert_typed_point(term, LinkType::Concept, None)
    }

    /// Inserts an object-identity point link.
    pub fn insert_object(&mut self, term: &str) -> LinkId {
        self.insert_typed_point(term, LinkType::Object, None)
    }

    /// Inserts a relation link with source span metadata.
    pub fn insert_relation<const N: usize>(
        &mut self,
        references: [LinkId; N],
        link_type: LinkType,
        span: SourceSpan,
    ) -> LinkId {
        self.insert_link(
            references,
            LinkMetadata::new()
                .with_link_type(link_type)
                .with_span(span),
        )
    }

    /// Inserts a labeled field relation as a regular link.
    pub fn insert_field(&mut self, parent: LinkId, label: &str, child: LinkId) -> LinkId {
        let label_link = self.insert_typed_point(
            label,
            LinkType::Field,
            Some("A field label names a relation between links."),
        );
        self.insert_link(
            [parent, label_link, child],
            LinkMetadata::new().with_link_type(LinkType::Field),
        )
    }

    /// Inserts a link from references and metadata.
    pub fn insert_link<const N: usize>(
        &mut self,
        references: [LinkId; N],
        metadata: LinkMetadata,
    ) -> LinkId {
        let id = self.allocate_id();
        self.links.insert(
            id,
            Link {
                id,
                references: references.to_vec(),
                metadata,
            },
        );
        id
    }

    /// Inserts a concept-to-language syntax mapping.
    pub fn insert_concept_mapping(
        &mut self,
        concept: &str,
        language: &str,
        syntax: &str,
    ) -> LinkId {
        let concept_link = self.insert_typed_point(
            concept,
            LinkType::Concept,
            Some("A concept mapping connects shared meaning to language syntax."),
        );
        let language_link = self.insert_typed_point(language, LinkType::Language, None);
        let mapping = self.insert_link(
            [concept_link, language_link],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_term(syntax)
                .with_language(language),
        );
        self.concept_syntax.insert(
            (concept.to_string(), language.to_string()),
            syntax.to_string(),
        );
        mapping
    }

    /// Reconstructs a concept using a target language syntax mapping.
    #[must_use]
    pub fn reconstruct_concept(&self, concept: &str, language: &str) -> Option<&str> {
        self.concept_syntax
            .get(&(concept.to_string(), language.to_string()))
            .map(String::as_str)
    }

    /// Applies a match-and-substitute rule over exact reference lists.
    pub fn apply_substitution(&mut self, rule: &SubstitutionRule) -> SubstitutionReport {
        let mut report = SubstitutionReport::default();

        if rule.pattern().is_empty() {
            if !rule.replacement().is_empty() {
                let created = self.insert_dynamic_link(
                    rule.replacement(),
                    LinkMetadata::new().with_link_type(LinkType::Relation),
                );
                report.created.push(created);
            }
            return report;
        }

        let matched = self
            .links()
            .filter(|link| link.references() == rule.pattern())
            .map(Link::id)
            .collect::<Vec<_>>();

        if rule.replacement().is_empty() {
            for id in matched {
                if self.links.remove(&id).is_some() {
                    report.deleted.push(id);
                }
            }
            return report;
        }

        for id in matched {
            if let Some(link) = self.links.get_mut(&id) {
                link.references = rule.replacement().to_vec();
                report.updated.push(id);
            }
        }

        report
    }

    /// Returns a link by id.
    #[must_use]
    pub fn link(&self, id: LinkId) -> Option<&Link> {
        self.links.get(&id)
    }

    /// Finds a self-description or named term link.
    #[must_use]
    pub fn find_term(&self, term: &str) -> Option<LinkId> {
        self.terms.get(term).copied()
    }

    /// Finds the definition attached to a term link.
    #[must_use]
    pub fn definition_for(&self, id: LinkId) -> Option<&str> {
        self.link(id).and_then(|link| link.metadata().definition())
    }

    /// Sets a source span on an existing link.
    pub fn set_span(&mut self, id: LinkId, span: SourceSpan) -> bool {
        let Some(link) = self.links.get_mut(&id) else {
            return false;
        };
        link.metadata_mut().span = Some(span);
        true
    }

    /// Sets parse flags on an existing link.
    pub fn set_flags(&mut self, id: LinkId, flags: LinkFlags) -> bool {
        let Some(link) = self.links.get_mut(&id) else {
            return false;
        };
        link.metadata_mut().flags = flags;
        true
    }

    /// Verifies that the selected region has no error or missing links.
    #[must_use]
    pub fn verify_full_match(&self, region: Option<ByteRange>) -> VerificationReport {
        let issues = self
            .links
            .values()
            .filter(|link| link_is_in_region(link, region))
            .filter_map(|link| {
                let flags = link.metadata().flags();
                let kind = if flags.is_error() {
                    VerificationIssueKind::ErrorLink
                } else if flags.is_missing() {
                    VerificationIssueKind::MissingLink
                } else if flags.has_error() {
                    VerificationIssueKind::HasErrorLink
                } else {
                    return None;
                };

                Some(VerificationIssue::new(
                    link.id(),
                    kind,
                    link.metadata().span(),
                ))
            })
            .collect();
        VerificationReport::new(issues)
    }

    fn insert_typed_point(
        &mut self,
        term: &str,
        link_type: LinkType,
        definition: Option<&str>,
    ) -> LinkId {
        if let Some(id) = self.terms.get(term).copied() {
            if let Some(definition) = definition {
                if let Some(link) = self.links.get_mut(&id) {
                    link.metadata_mut().definition = Some(definition.to_string());
                }
            }
            return id;
        }

        let id = self.allocate_id();
        let mut metadata = LinkMetadata::new()
            .with_link_type(link_type)
            .with_named(true)
            .with_term(term);
        if let Some(definition) = definition {
            metadata = metadata.with_definition(definition);
        }
        self.links.insert(
            id,
            Link {
                id,
                references: vec![id],
                metadata,
            },
        );
        self.terms.insert(term.to_string(), id);
        id
    }

    pub(crate) fn attach_trivia(
        &mut self,
        document: LinkId,
        token: LinkId,
        span: SourceSpan,
        policy: TriviaAttachmentPolicy,
    ) {
        match policy {
            TriviaAttachmentPolicy::ContainmentLink => {
                self.insert_containment_trivia(document, token, span);
            }
            TriviaAttachmentPolicy::TokenLink => {
                self.insert_token_trivia(token, span);
            }
            TriviaAttachmentPolicy::Both => {
                self.insert_containment_trivia(document, token, span);
                self.insert_token_trivia(token, span);
            }
        }
    }

    fn insert_containment_trivia(&mut self, document: LinkId, token: LinkId, span: SourceSpan) {
        self.insert_link(
            [document, token],
            LinkMetadata::new()
                .with_link_type(LinkType::Trivia)
                .with_term("containment trivia")
                .with_span(span)
                .with_flags(LinkFlags::extra()),
        );
    }

    fn insert_token_trivia(&mut self, token: LinkId, span: SourceSpan) {
        self.insert_link(
            [token],
            LinkMetadata::new()
                .with_link_type(LinkType::Trivia)
                .with_term("token trivia")
                .with_span(span)
                .with_flags(LinkFlags::extra()),
        );
    }

    fn insert_dynamic_link(&mut self, references: &[LinkId], metadata: LinkMetadata) -> LinkId {
        let id = self.allocate_id();
        self.links.insert(
            id,
            Link {
                id,
                references: references.to_vec(),
                metadata,
            },
        );
        id
    }

    pub(crate) fn attach_embedded_regions(
        &mut self,
        document: LinkId,
        text: &str,
        language: &str,
        policy: RegionDetectionPolicy,
    ) {
        for region in detect_embedded_regions(text, language, policy) {
            let region_language = region.language().to_string();
            let language_link = self.insert_typed_point(&region_language, LinkType::Language, None);
            self.insert_link(
                [document, language_link],
                LinkMetadata::new()
                    .with_link_type(LinkType::Region)
                    .with_named(true)
                    .with_term(format!("{region_language} region"))
                    .with_language(region_language)
                    .with_span(region.span()),
            );
        }
    }

    const fn allocate_id(&mut self) -> LinkId {
        let id = LinkId(self.next_id);
        self.next_id += 1;
        id
    }
}

fn link_is_in_region(link: &Link, region: Option<ByteRange>) -> bool {
    let Some(region) = region else {
        return true;
    };
    link.metadata()
        .span()
        .is_some_and(|span| span.byte_range().intersects(region))
}

fn end_point_for_text(text: &str) -> Point {
    let mut row = 0;
    let mut column = 0;
    for character in text.chars() {
        if character == '\n' {
            row += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    Point::new(row, column)
}
