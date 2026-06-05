use std::collections::BTreeMap;
use std::fmt;

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

/// Half-open byte range in source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteRange {
    start: usize,
    end: usize,
}

impl ByteRange {
    /// Creates a byte range.
    ///
    /// # Panics
    ///
    /// Panics when `start` is greater than `end`.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "byte range start must not exceed end");
        Self { start, end }
    }

    /// First byte in the range.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Byte immediately after the range.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns `true` when this range intersects `other`.
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        if self.start == self.end || other.start == other.end {
            self.start <= other.end && other.start <= self.end
        } else {
            self.start < other.end && other.start < self.end
        }
    }
}

/// Row and column point in source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    row: usize,
    column: usize,
}

impl Point {
    /// Creates a row/column point.
    #[must_use]
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    /// Zero-based row.
    #[must_use]
    pub const fn row(self) -> usize {
        self.row
    }

    /// Zero-based column.
    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }
}

/// Source span attached to a link.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    byte_range: ByteRange,
    start_point: Point,
    end_point: Point,
}

impl SourceSpan {
    /// Creates a source span from byte and point ranges.
    #[must_use]
    pub const fn new(byte_range: ByteRange, start_point: Point, end_point: Point) -> Self {
        Self {
            byte_range,
            start_point,
            end_point,
        }
    }

    /// Byte range covered by the span.
    #[must_use]
    pub const fn byte_range(self) -> ByteRange {
        self.byte_range
    }

    /// Start row/column point.
    #[must_use]
    pub const fn start_point(self) -> Point {
        self.start_point
    }

    /// End row/column point.
    #[must_use]
    pub const fn end_point(self) -> Point {
        self.end_point
    }
}

/// Tree-sitter-compatible parse status flags modeled as link metadata.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LinkFlags {
    bits: u8,
}

impl LinkFlags {
    const IS_ERROR: u8 = 0b0001;
    const HAS_ERROR: u8 = 0b0010;
    const IS_MISSING: u8 = 0b0100;
    const IS_EXTRA: u8 = 0b1000;

    /// Clean link flags.
    #[must_use]
    pub const fn clean() -> Self {
        Self { bits: 0 }
    }

    /// Flags for an error link.
    #[must_use]
    pub const fn error() -> Self {
        Self {
            bits: Self::IS_ERROR,
        }
    }

    /// Flags for a link that contains an error below it.
    #[must_use]
    pub const fn containing_error() -> Self {
        Self {
            bits: Self::HAS_ERROR,
        }
    }

    /// Flags for a missing link.
    #[must_use]
    pub const fn missing() -> Self {
        Self {
            bits: Self::IS_MISSING,
        }
    }

    /// Flags for an extra/trivia link.
    #[must_use]
    pub const fn extra() -> Self {
        Self {
            bits: Self::IS_EXTRA,
        }
    }

    /// Whether this link is an error link.
    #[must_use]
    pub const fn is_error(self) -> bool {
        self.bits & Self::IS_ERROR != 0
    }

    /// Whether this link contains an error below it.
    #[must_use]
    pub const fn has_error(self) -> bool {
        self.bits & Self::HAS_ERROR != 0
    }

    /// Whether this link is missing from the source text.
    #[must_use]
    pub const fn is_missing(self) -> bool {
        self.bits & Self::IS_MISSING != 0
    }

    /// Whether this link is extra source trivia.
    #[must_use]
    pub const fn is_extra(self) -> bool {
        self.bits & Self::IS_EXTRA != 0
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

/// Trivia attachment strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriviaAttachmentPolicy {
    /// Attach trivia to the containing syntax link.
    ContainmentLink,
    /// Attach trivia to the token link.
    TokenLink,
    /// Emit both attachment links when they can coexist.
    Both,
}

/// Region detection strategy for mixed-language parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionDetectionPolicy {
    /// Use explicit region names such as fenced-code language tags.
    NameDriven,
    /// Use content sniffing.
    ContentDriven,
    /// Use name-driven detection first and content-driven detection as a fallback.
    Both,
}

/// Configuration for parse-to-network operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseConfiguration {
    trivia_attachment_policy: TriviaAttachmentPolicy,
    region_detection_policy: RegionDetectionPolicy,
}

impl ParseConfiguration {
    /// Creates parse configuration with the supplied trivia policy.
    #[must_use]
    pub const fn new(trivia_attachment_policy: TriviaAttachmentPolicy) -> Self {
        Self {
            trivia_attachment_policy,
            region_detection_policy: RegionDetectionPolicy::Both,
        }
    }

    /// Returns configuration with a mixed-language region detection policy.
    #[must_use]
    pub const fn with_region_detection_policy(
        mut self,
        region_detection_policy: RegionDetectionPolicy,
    ) -> Self {
        self.region_detection_policy = region_detection_policy;
        self
    }

    /// Trivia attachment policy.
    #[must_use]
    pub const fn trivia_attachment_policy(self) -> TriviaAttachmentPolicy {
        self.trivia_attachment_policy
    }

    /// Mixed-language region detection policy.
    #[must_use]
    pub const fn region_detection_policy(self) -> RegionDetectionPolicy {
        self.region_detection_policy
    }
}

impl Default for ParseConfiguration {
    fn default() -> Self {
        Self::new(TriviaAttachmentPolicy::Both)
    }
}

/// Verification issue kind for a full-match check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationIssueKind {
    /// A link explicitly marks a parse error.
    ErrorLink,
    /// A link marks source text missing from the parse.
    MissingLink,
    /// A link contains a parse error below it.
    HasErrorLink,
}

/// One verification issue tied to a link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationIssue {
    link_id: LinkId,
    kind: VerificationIssueKind,
    span: Option<SourceSpan>,
}

impl VerificationIssue {
    /// Link that caused the issue.
    #[must_use]
    pub const fn link_id(&self) -> LinkId {
        self.link_id
    }

    /// Issue kind.
    #[must_use]
    pub const fn kind(&self) -> VerificationIssueKind {
        self.kind
    }

    /// Source span attached to the issue link, when available.
    #[must_use]
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }
}

/// Result of verifying that a source region fully matches a language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationReport {
    issues: Vec<VerificationIssue>,
}

impl VerificationReport {
    /// Whether the verified region has no error or missing links.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }

    /// Verification issues found in the region.
    #[must_use]
    pub fn issues(&self) -> &[VerificationIssue] {
        &self.issues
    }
}

/// Mutable links network for CST, AST, semantic, and self-description links.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkNetwork {
    next_id: u64,
    links: BTreeMap<LinkId, Link>,
    terms: BTreeMap<String, LinkId>,
}

impl LinkNetwork {
    /// Creates an empty links network.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            next_id: 1,
            links: BTreeMap::new(),
            terms: BTreeMap::new(),
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
        network
    }

    /// Parses plain source text into a lossless token network.
    ///
    /// This is a minimal parser boundary: it preserves source spans and trivia
    /// links while language-specific parsers are added behind the same network
    /// representation.
    #[must_use]
    pub fn parse_lossless_text(
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> Self {
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

        let mut row = 0;
        let mut column = 0;
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
            if character.is_whitespace() {
                network.attach_trivia(
                    document,
                    token,
                    span,
                    configuration.trivia_attachment_policy,
                );
            }
        }

        network
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

    /// Inserts a self-referential point link for a term.
    pub fn insert_point(&mut self, term: &str) -> LinkId {
        self.insert_typed_point(term, LinkType::Concept, None)
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

                Some(VerificationIssue {
                    link_id: link.id(),
                    kind,
                    span: link.metadata().span(),
                })
            })
            .collect();
        VerificationReport { issues }
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

    fn attach_trivia(
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
