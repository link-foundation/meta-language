//! API-style parity registry and adapters.
//!
//! The registry makes issue #62's "same operations through every applicable
//! style" requirement executable: each operation has an explicit cell for each
//! supported style, and applicable cells point at a runnable fixture.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use links_notation::{parse_lino_to_links, LiNo};

use crate::configuration::ParseConfiguration;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::query::LinkQuery;
use crate::snapshots::NetworkSnapshot;
use crate::source::ByteRange;
use crate::substitution::{SubstitutionReport, SubstitutionRule};
use crate::transform::{ReplacementReport, ReplacementRule};
use crate::translation_rules::TranslationRuleSet;
use crate::verification::VerificationReport;

mod fixtures;

pub use fixtures::run_api_style_fixture;

/// Operation families that must remain reachable through the supported API
/// styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiOperation {
    /// Parse source text into a links network.
    Parse,
    /// Query links in a network.
    Query,
    /// Transform query-selected links or source ranges.
    Transform,
    /// Apply structural substitutions.
    Substitute,
    /// Serialize and load network data.
    Serialize,
    /// Capture immutable network versions.
    Snapshot,
    /// Reconstruct through translation rules.
    Translate,
    /// Verify parse and structural diagnostics.
    Verify,
}

impl ApiOperation {
    /// Stable registry label.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Query => "query",
            Self::Transform => "transform",
            Self::Substitute => "substitute",
            Self::Serialize => "serialize",
            Self::Snapshot => "snapshot",
            Self::Translate => "translate",
            Self::Verify => "verify",
        }
    }
}

/// API surface styles tracked for parity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiStyle {
    /// Direct Rust methods on core types.
    DirectMethod,
    /// Fluent chain over the same executor methods.
    FluentChain,
    /// link-cli-compatible substitution text.
    LinkCliSubstitutionText,
    /// S-expression or `LiNo` text surfaces.
    SexpressionOrLinoText,
}

impl ApiStyle {
    /// All styles that must appear in every operation row.
    pub const ALL: &'static [Self] = &[
        Self::DirectMethod,
        Self::FluentChain,
        Self::LinkCliSubstitutionText,
        Self::SexpressionOrLinoText,
    ];
}

/// Coverage state for one operation/style cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApiStyleFixture {
    /// The style applies and is covered by a runnable fixture.
    Executable(&'static str),
    /// The style does not apply to this operation, with an explicit reason.
    NotApplicable(&'static str),
}

/// One operation/style registry cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApiStyleCell {
    style: ApiStyle,
    fixture: ApiStyleFixture,
}

impl ApiStyleCell {
    /// Creates a cell covered by an executable fixture.
    #[must_use]
    pub const fn executable(style: ApiStyle, fixture_name: &'static str) -> Self {
        Self {
            style,
            fixture: ApiStyleFixture::Executable(fixture_name),
        }
    }

    /// Creates an explicit N/A cell.
    #[must_use]
    pub const fn not_applicable(style: ApiStyle, reason: &'static str) -> Self {
        Self {
            style,
            fixture: ApiStyleFixture::NotApplicable(reason),
        }
    }

    /// Style represented by this cell.
    #[must_use]
    pub const fn style(self) -> ApiStyle {
        self.style
    }

    /// Fixture coverage for this cell.
    #[must_use]
    pub const fn fixture(self) -> ApiStyleFixture {
        self.fixture
    }
}

/// One operation row in the API-style parity matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApiOperationEntry {
    operation: ApiOperation,
    styles: &'static [ApiStyleCell],
}

impl ApiOperationEntry {
    /// Operation represented by this row.
    #[must_use]
    pub const fn operation(self) -> ApiOperation {
        self.operation
    }

    /// Stable operation label.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.operation.name()
    }

    /// Style cells in this row.
    #[must_use]
    pub const fn styles(self) -> &'static [ApiStyleCell] {
        self.styles
    }

    /// Returns the cell for `style`, when present.
    #[must_use]
    pub fn style(self, style: ApiStyle) -> Option<ApiStyleCell> {
        self.styles
            .iter()
            .copied()
            .find(|cell| cell.style() == style)
    }
}

const LINK_CLI_PARSE_NA: &str =
    "link-cli substitution text mutates existing links; it is not a source parser";
const LINK_CLI_SERIALIZE_NA: &str =
    "link-cli substitution text is an operation command, not a network serializer";
const LINK_CLI_SNAPSHOT_NA: &str =
    "link-cli substitution text has no immutable versioning primitive";
const LINK_CLI_TRANSLATE_NA: &str =
    "link-cli substitution text rewrites links and does not select target languages";
const LINK_CLI_VERIFY_NA: &str =
    "link-cli substitution text has no diagnostic verification primitive";
const TEXT_SNAPSHOT_NA: &str =
    "snapshots carry runtime version provenance and are not a standalone text DSL";
const TEXT_VERIFY_NA: &str =
    "verification consumes an existing network rather than a standalone text DSL";

const PARSE_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "parse.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "parse.fluent"),
    ApiStyleCell::not_applicable(ApiStyle::LinkCliSubstitutionText, LINK_CLI_PARSE_NA),
    ApiStyleCell::executable(ApiStyle::SexpressionOrLinoText, "parse.lino_text"),
];

const QUERY_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "query.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "query.fluent"),
    ApiStyleCell::executable(
        ApiStyle::LinkCliSubstitutionText,
        "query.link_cli_read_identity",
    ),
    ApiStyleCell::executable(ApiStyle::SexpressionOrLinoText, "query.sexpression"),
];

const TRANSFORM_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "transform.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "transform.fluent"),
    ApiStyleCell::executable(
        ApiStyle::LinkCliSubstitutionText,
        "transform.link_cli_update",
    ),
    ApiStyleCell::executable(ApiStyle::SexpressionOrLinoText, "transform.sexpression"),
];

const SUBSTITUTE_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "substitute.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "substitute.fluent"),
    ApiStyleCell::executable(
        ApiStyle::LinkCliSubstitutionText,
        "substitute.link_cli_crud",
    ),
    ApiStyleCell::executable(ApiStyle::SexpressionOrLinoText, "substitute.lino_text"),
];

const SERIALIZE_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "serialize.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "serialize.fluent"),
    ApiStyleCell::not_applicable(ApiStyle::LinkCliSubstitutionText, LINK_CLI_SERIALIZE_NA),
    ApiStyleCell::executable(ApiStyle::SexpressionOrLinoText, "serialize.lino_roundtrip"),
];

const SNAPSHOT_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "snapshot.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "snapshot.fluent"),
    ApiStyleCell::not_applicable(ApiStyle::LinkCliSubstitutionText, LINK_CLI_SNAPSHOT_NA),
    ApiStyleCell::not_applicable(ApiStyle::SexpressionOrLinoText, TEXT_SNAPSHOT_NA),
];

const TRANSLATE_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "translate.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "translate.fluent"),
    ApiStyleCell::not_applicable(ApiStyle::LinkCliSubstitutionText, LINK_CLI_TRANSLATE_NA),
    ApiStyleCell::executable(ApiStyle::SexpressionOrLinoText, "translate.lino_rules"),
];

const VERIFY_STYLES: &[ApiStyleCell] = &[
    ApiStyleCell::executable(ApiStyle::DirectMethod, "verify.direct"),
    ApiStyleCell::executable(ApiStyle::FluentChain, "verify.fluent"),
    ApiStyleCell::not_applicable(ApiStyle::LinkCliSubstitutionText, LINK_CLI_VERIFY_NA),
    ApiStyleCell::not_applicable(ApiStyle::SexpressionOrLinoText, TEXT_VERIFY_NA),
];

/// Operation/style parity matrix.
pub const API_OPERATIONS: &[ApiOperationEntry] = &[
    ApiOperationEntry {
        operation: ApiOperation::Parse,
        styles: PARSE_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Query,
        styles: QUERY_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Transform,
        styles: TRANSFORM_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Substitute,
        styles: SUBSTITUTE_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Serialize,
        styles: SERIALIZE_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Snapshot,
        styles: SNAPSHOT_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Translate,
        styles: TRANSLATE_STYLES,
    },
    ApiOperationEntry {
        operation: ApiOperation::Verify,
        styles: VERIFY_STYLES,
    },
];

/// Fluent adapter over [`LinkNetwork`] operations.
pub trait FluentNetworkApi: Sized {
    /// Converts `self` into the underlying network executor.
    fn into_network(self) -> LinkNetwork;

    /// Starts a fluent chain over the same network executor.
    #[must_use]
    fn into_fluent(self) -> FluentPipeline {
        FluentPipeline::new(self.into_network())
    }
}

impl FluentNetworkApi for LinkNetwork {
    fn into_network(self) -> LinkNetwork {
        self
    }
}

/// Fluent parse/query/transform/reconstruct pipeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FluentPipeline {
    network: LinkNetwork,
    matches: Vec<crate::query::QueryMatch>,
    last_report: ReplacementReport,
}

impl FluentPipeline {
    /// Starts a fluent chain from an existing network.
    #[must_use]
    pub fn new(network: LinkNetwork) -> Self {
        Self {
            network,
            matches: Vec::new(),
            last_report: ReplacementReport::default(),
        }
    }

    /// Parses source text and starts a fluent chain.
    #[must_use]
    pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> Self {
        Self::new(LinkNetwork::parse(text, language, configuration))
    }

    /// Selects links with a structural query.
    #[must_use]
    pub fn find(mut self, query: impl Into<LinkQuery>) -> Self {
        let query = query.into();
        self.matches = self.network.find(&query);
        self
    }

    /// Replaces links selected by the most recent [`Self::find`] call.
    #[must_use]
    pub fn replace(mut self, rule: impl Into<ReplacementRule>) -> Self {
        let rule = rule.into();
        self.last_report = self.network.replace(&self.matches, &rule);
        self
    }

    /// Applies a structural substitution rule.
    #[must_use]
    pub fn substitute(mut self, rule: impl Into<SubstitutionRule>) -> Self {
        let rule = rule.into();
        self.last_report = report_from_substitution(self.network.apply_substitution(&rule));
        self
    }

    /// Applies a link-cli-style substitution command.
    ///
    /// # Errors
    ///
    /// Returns [`LinkCliSubstitutionError`] when the command text is malformed.
    pub fn link_cli_substitution_text(
        mut self,
        source: &str,
    ) -> Result<Self, LinkCliSubstitutionError> {
        self.last_report =
            report_from_substitution(self.network.apply_link_cli_substitution_text(source)?);
        Ok(self)
    }

    /// Reconstructs source text from the current network.
    #[must_use]
    pub fn reconstruct(self) -> String {
        self.network.reconstruct_text()
    }

    /// Serializes the current network to canonical `LiNo` text.
    #[must_use]
    pub fn serialize(&self) -> String {
        self.network.to_lino()
    }

    /// Captures an immutable snapshot of the current network.
    #[must_use]
    pub fn snapshot(&self, version: u64, provenance: impl Into<String>) -> NetworkSnapshot {
        self.network.snapshot(version, provenance)
    }

    /// Reconstructs text for a target language.
    #[must_use]
    pub fn translate(
        &self,
        target_language: &str,
        configuration: ParseConfiguration,
        rules: &TranslationRuleSet,
    ) -> String {
        self.network
            .reconstruct_text_as_with_rules(target_language, configuration, rules)
    }

    /// Verifies the current network.
    #[must_use]
    pub fn verify(&self, region: Option<ByteRange>) -> VerificationReport {
        self.network.verify_full_match(region)
    }

    /// Last transform or substitution report.
    #[must_use]
    pub const fn last_report(&self) -> &ReplacementReport {
        &self.last_report
    }

    /// Borrows the current network.
    #[must_use]
    pub const fn network(&self) -> &LinkNetwork {
        &self.network
    }

    /// Ends the fluent chain and returns the current network.
    #[must_use]
    pub fn into_network(self) -> LinkNetwork {
        self.network
    }
}

impl LinkNetwork {
    /// Parses source text and starts a fluent chain.
    #[must_use]
    pub fn parse_fluent(
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> FluentPipeline {
        FluentPipeline::parse(text, language, configuration)
    }

    /// Applies a link-cli-style substitution command.
    ///
    /// # Errors
    ///
    /// Returns [`LinkCliSubstitutionError`] when the command text is malformed.
    pub fn apply_link_cli_substitution_text(
        &mut self,
        source: &str,
    ) -> Result<SubstitutionReport, LinkCliSubstitutionError> {
        LinkCliSubstitution::parse(source)?.apply(self)
    }
}

/// Operation kind represented by a link-cli-style substitution command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkCliSubstitutionKind {
    /// Empty match side creates replacement links.
    Create,
    /// Identical match and replacement sides read/echo matches without changing references.
    ReadIdentity,
    /// Non-empty match and replacement sides update matched links.
    Update,
    /// Empty replacement side deletes matched links.
    Delete,
}

/// Parsed link-cli-style substitution command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkCliSubstitution {
    pattern: Vec<LinkCliLinkPattern>,
    replacement: Vec<LinkCliLinkPattern>,
}

impl LinkCliSubstitution {
    /// Parses `(match) (substitution)` `LiNo` text.
    ///
    /// # Errors
    ///
    /// Returns [`LinkCliSubstitutionError`] when the text is not a two-sided
    /// link-cli substitution command.
    pub fn parse(source: &str) -> Result<Self, LinkCliSubstitutionError> {
        let statements = parse_lino_to_links(source)
            .map_err(|error| LinkCliSubstitutionError::new(error.to_string()))?;
        let (pattern, replacement) = match statements.as_slice() {
            [pattern, replacement] => (pattern, replacement),
            [LiNo::Link { id: None, values }] if values.len() == 2 => (&values[0], &values[1]),
            _ => {
                return Err(LinkCliSubstitutionError::new(
                    "link-cli substitution requires exactly two LiNo lists",
                ))
            }
        };

        Ok(Self {
            pattern: parse_substitution_side(pattern, "match")?,
            replacement: parse_substitution_side(replacement, "replacement")?,
        })
    }

    /// Builds a link id from a numeric link-cli reference.
    #[must_use]
    pub const fn link_id(value: u64) -> LinkId {
        LinkId::from_u64(value)
    }

    /// Classifies this command.
    #[must_use]
    pub fn kind(&self) -> LinkCliSubstitutionKind {
        match (self.pattern.is_empty(), self.replacement.is_empty()) {
            (true, false) => LinkCliSubstitutionKind::Create,
            (false, true) => LinkCliSubstitutionKind::Delete,
            (false, false) if self.pattern == self.replacement => {
                LinkCliSubstitutionKind::ReadIdentity
            }
            _ => LinkCliSubstitutionKind::Update,
        }
    }

    /// Applies this command to `network`.
    ///
    /// # Errors
    ///
    /// Currently reserved for malformed parsed states; valid parsed commands
    /// apply infallibly.
    pub fn apply(
        &self,
        network: &mut LinkNetwork,
    ) -> Result<SubstitutionReport, LinkCliSubstitutionError> {
        let report = match self.kind() {
            LinkCliSubstitutionKind::Create => self.apply_create(network),
            LinkCliSubstitutionKind::Delete => self.apply_delete(network),
            LinkCliSubstitutionKind::ReadIdentity | LinkCliSubstitutionKind::Update => {
                self.apply_update(network)
            }
        };
        Ok(report)
    }

    fn apply_create(&self, network: &mut LinkNetwork) -> SubstitutionReport {
        let mut report = SubstitutionReport::default();
        for replacement in &self.replacement {
            let created = network.insert_dynamic_link(
                &replacement.references,
                LinkMetadata::new().with_link_type(LinkType::Relation),
            );
            report.created.push(created);
        }
        report
    }

    fn apply_delete(&self, network: &mut LinkNetwork) -> SubstitutionReport {
        let mut report = SubstitutionReport::default();
        for id in self.matching_ids(network) {
            if network.links.remove(&id).is_some() {
                report.deleted.push(id);
            }
        }
        report
    }

    fn apply_update(&self, network: &mut LinkNetwork) -> SubstitutionReport {
        let mut report = SubstitutionReport::default();
        for (pattern, replacement) in self.pattern.iter().zip(&self.replacement) {
            for id in matching_ids_for_pattern(network, pattern) {
                if replacement
                    .id
                    .is_some_and(|replacement_id| replacement_id != id)
                {
                    continue;
                }
                if network.set_references(id, &replacement.references) {
                    report.updated.push(id);
                }
            }
        }
        report
    }

    fn matching_ids(&self, network: &LinkNetwork) -> Vec<LinkId> {
        let mut seen = BTreeSet::new();
        let mut ids = Vec::new();
        for pattern in &self.pattern {
            for id in matching_ids_for_pattern(network, pattern) {
                if seen.insert(id) {
                    ids.push(id);
                }
            }
        }
        ids
    }
}

/// Error returned while parsing or applying link-cli-style substitution text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkCliSubstitutionError {
    message: String,
}

impl LinkCliSubstitutionError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for LinkCliSubstitutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for LinkCliSubstitutionError {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinkCliLinkPattern {
    id: Option<LinkId>,
    references: Vec<LinkId>,
}

fn parse_substitution_side(
    node: &LiNo<String>,
    label: &str,
) -> Result<Vec<LinkCliLinkPattern>, LinkCliSubstitutionError> {
    let LiNo::Link { id: None, values } = node else {
        return Err(LinkCliSubstitutionError::new(format!(
            "{label} side must be an anonymous LiNo list"
        )));
    };

    values
        .iter()
        .map(|value| parse_link_pattern(value, label))
        .collect()
}

fn parse_link_pattern(
    node: &LiNo<String>,
    label: &str,
) -> Result<LinkCliLinkPattern, LinkCliSubstitutionError> {
    let LiNo::Link { id, values } = node else {
        return Err(LinkCliSubstitutionError::new(format!(
            "{label} side entries must be LiNo links"
        )));
    };

    let id = id.as_deref().map(parse_link_id).transpose()?;
    let references = values
        .iter()
        .map(parse_link_reference)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(LinkCliLinkPattern { id, references })
}

fn parse_link_reference(node: &LiNo<String>) -> Result<LinkId, LinkCliSubstitutionError> {
    let LiNo::Ref(reference) = node else {
        return Err(LinkCliSubstitutionError::new(
            "link-cli substitution references must be numeric refs",
        ));
    };
    parse_link_id(reference)
}

fn parse_link_id(value: &str) -> Result<LinkId, LinkCliSubstitutionError> {
    value
        .parse::<u64>()
        .map(LinkId::from_u64)
        .map_err(|_| LinkCliSubstitutionError::new(format!("invalid link id `{value}`")))
}

fn matching_ids_for_pattern(network: &LinkNetwork, pattern: &LinkCliLinkPattern) -> Vec<LinkId> {
    if let Some(id) = pattern.id {
        return network
            .link(id)
            .filter(|link| link.references() == pattern.references.as_slice())
            .map(|link| vec![link.id()])
            .unwrap_or_default();
    }

    network
        .links()
        .filter(|link| link.references() == pattern.references.as_slice())
        .map(crate::link_network::Link::id)
        .collect()
}

const fn report_from_substitution(substitution: SubstitutionReport) -> ReplacementReport {
    ReplacementReport::from_substitution(substitution)
}
