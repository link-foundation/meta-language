//! Engine-neutral executable query plans and pluggable source frontends.
//!
//! A plan contains canonical operation semantics plus separate source evidence.
//! Consequently, equivalent source dialects have equal [`QueryOperation`]s
//! without discarding the language and CST link that produced each plan.

mod sql;

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use serde::Serialize;

use crate::{
    ByteRange, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, Point, SourceSpan,
};

/// SQL profiles whose common subset is normalized by the built-in frontend.
pub const SQL_DIALECT_PROFILES: &[SqlDialectProfile] = &[
    SqlDialectProfile::new("sql-ansi", "ANSI SQL"),
    SqlDialectProfile::new("sql-postgres", "PostgreSQL"),
    SqlDialectProfile::new("sql-mysql", "MySQL"),
    SqlDialectProfile::new("sql-sqlite", "SQLite"),
    SqlDialectProfile::new("sql-server", "SQL Server"),
    SqlDialectProfile::new("sql-oracle", "Oracle"),
    SqlDialectProfile::new("sql-bigquery", "BigQuery"),
    SqlDialectProfile::new("sql-snowflake", "Snowflake"),
];

/// A registered SQL vendor profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SqlDialectProfile {
    key: &'static str,
    vendor: &'static str,
}

impl SqlDialectProfile {
    const fn new(key: &'static str, vendor: &'static str) -> Self {
        Self { key, vendor }
    }

    /// Registry/language key used at the API boundary.
    #[must_use]
    pub const fn key(self) -> &'static str {
        self.key
    }

    /// Human-readable vendor or standard name.
    #[must_use]
    pub const fn vendor(self) -> &'static str {
        self.vendor
    }

    fn lookup(key: &str) -> Option<Self> {
        SQL_DIALECT_PROFILES
            .iter()
            .copied()
            .find(|profile| key.eq_ignore_ascii_case(profile.key))
    }
}

/// Whether a lowered plan has been authorized for execution.
///
/// Lowering never authorizes a statement. Consumers must apply their own
/// identity, data-access, resource, and mutation policies before execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryAuthorization {
    /// An execution policy must still approve this plan.
    Required,
}

/// Source/CST provenance retained separately from canonical semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceEvidence {
    language: String,
    span: SourceSpan,
    syntax_link: Option<LinkId>,
}

impl SourceEvidence {
    /// Creates source evidence.
    #[must_use]
    pub fn new(language: impl Into<String>, span: SourceSpan, syntax_link: Option<LinkId>) -> Self {
        Self {
            language: language.into(),
            span,
            syntax_link,
        }
    }

    /// Creates evidence for a frontend that has no source byte range or CST.
    #[must_use]
    pub fn synthetic(language: impl Into<String>) -> Self {
        Self::new(
            language,
            SourceSpan::new(ByteRange::new(0, 0), Point::new(0, 0), Point::new(0, 0)),
            None,
        )
    }

    /// Source language/profile key.
    #[must_use]
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Source span covering the lowered statement.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    /// CST syntax link that supplied the statement, when available.
    #[must_use]
    pub const fn syntax_link(&self) -> Option<LinkId> {
        self.syntax_link
    }
}

/// Engine-neutral table or relation source.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySource {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

/// A projected expression and its optional result alias.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Projection {
    pub expression: QueryExpression,
    pub alias: Option<String>,
}

/// An update assignment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub column: Vec<String>,
    pub value: QueryExpression,
}

/// An ordering expression.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortExpression {
    pub expression: QueryExpression,
    pub direction: SortDirection,
}

/// Sort direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Scalar literal in a plan.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QueryValue {
    Null,
    Boolean { value: bool },
    Number { value: String },
    String { value: String },
}

/// Supported canonical aggregate functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregateFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    VariancePopulation,
    StandardDeviationPopulation,
}

impl AggregateFunction {
    const fn label(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Sum => "sum",
            Self::Avg => "avg",
            Self::Min => "min",
            Self::Max => "max",
            Self::VariancePopulation => "variance_population",
            Self::StandardDeviationPopulation => "standard_deviation_population",
        }
    }
}

/// Canonical unary expression operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnaryOperator {
    Not,
    Negate,
    Positive,
}

/// Canonical binary expression operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOperator {
    Or,
    And,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Is,
    IsNot,
    Like,
    NotLike,
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Canonical query expression.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QueryExpression {
    Column {
        path: Vec<String>,
    },
    Literal {
        value: QueryValue,
    },
    Parameter {
        name: String,
    },
    Wildcard,
    Unary {
        operator: UnaryOperator,
        operand: Box<Self>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Self>,
        right: Box<Self>,
    },
    Aggregate {
        function: AggregateFunction,
        expression: Box<Self>,
        distinct: bool,
    },
    Function {
        name: String,
        arguments: Vec<Self>,
    },
    /// Explicit extension node for a custom/vendor frontend.
    Extension {
        namespace: String,
        name: String,
        arguments: Vec<Self>,
    },
}

/// Canonical CRUD/query operation, independent of source language and engine.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QueryOperation {
    Select {
        distinct: bool,
        projection: Vec<Projection>,
        #[serde(rename = "from")]
        source: Option<QuerySource>,
        #[serde(rename = "filter")]
        predicate: Option<QueryExpression>,
        #[serde(rename = "groupBy")]
        group_by: Vec<QueryExpression>,
        #[serde(rename = "orderBy")]
        order_by: Vec<SortExpression>,
        limit: Option<u64>,
        offset: Option<u64>,
    },
    Insert {
        into: QuerySource,
        columns: Vec<String>,
        rows: Vec<Vec<QueryExpression>>,
    },
    Update {
        table: QuerySource,
        assignments: Vec<Assignment>,
        #[serde(rename = "filter")]
        predicate: Option<QueryExpression>,
    },
    Delete {
        #[serde(rename = "from")]
        source: QuerySource,
        #[serde(rename = "filter")]
        predicate: Option<QueryExpression>,
    },
}

impl QueryOperation {
    const fn label(&self) -> &'static str {
        match self {
            Self::Select { .. } => "select",
            Self::Insert { .. } => "insert",
            Self::Update { .. } => "update",
            Self::Delete { .. } => "delete",
        }
    }
}

/// A canonical operation with its non-authorizing source evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryPlan {
    operation: QueryOperation,
    evidence: SourceEvidence,
}

impl QueryPlan {
    /// Creates a plan produced by a source frontend.
    #[must_use]
    pub const fn new(operation: QueryOperation, evidence: SourceEvidence) -> Self {
        Self {
            operation,
            evidence,
        }
    }

    /// Canonical semantics, excluding source-specific provenance.
    #[must_use]
    pub const fn operation(&self) -> &QueryOperation {
        &self.operation
    }

    /// Source/CST evidence.
    #[must_use]
    pub const fn evidence(&self) -> &SourceEvidence {
        &self.evidence
    }

    /// Lowering never authorizes execution.
    #[must_use]
    pub const fn authorization(&self) -> QueryAuthorization {
        QueryAuthorization::Required
    }

    /// Stable language-neutral JSON form used by cross-runtime fixtures.
    #[must_use]
    pub fn canonical_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.operation).expect("query operation serialization is infallible")
    }

    /// Declares the plan concepts as semantic links in `network`.
    pub fn declare_in(&self, network: &mut LinkNetwork) -> QueryPlanLinks {
        let definition = self.canonical_json().to_string();
        let metadata = LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("query-plan")
            .with_definition(&definition);
        let root = if let Some(syntax) = self
            .evidence
            .syntax_link
            .filter(|id| network.link(*id).is_some())
        {
            network.insert_link([syntax], metadata)
        } else {
            network.insert_link([], metadata)
        };
        let mut links = vec![root];
        let operation = insert_semantic(
            network,
            root,
            &format!("query-operation:{}", self.operation.label()),
            None,
            &mut links,
        );
        declare_operation(network, operation, &self.operation, &mut links);
        QueryPlanLinks { root, links }
    }
}

/// Link identifiers created by [`QueryPlan::declare_in`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryPlanLinks {
    root: LinkId,
    links: Vec<LinkId>,
}

impl QueryPlanLinks {
    /// Root semantic plan link.
    #[must_use]
    pub const fn root(&self) -> LinkId {
        self.root
    }

    /// Root and all descendant semantic links.
    #[must_use]
    pub fn links(&self) -> &[LinkId] {
        &self.links
    }
}

/// Error category for fail-closed frontend lowering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryPlanErrorKind {
    UnsupportedLanguage,
    InvalidConcreteSyntax,
    Syntax,
    Semantic,
    Frontend,
}

/// Error returned when source cannot safely become a query plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryPlanError {
    kind: QueryPlanErrorKind,
    message: String,
    offset: Option<usize>,
}

impl QueryPlanError {
    pub(crate) fn new(
        kind: QueryPlanErrorKind,
        message: impl Into<String>,
        offset: Option<usize>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            offset,
        }
    }

    /// Creates an error reported by a custom frontend.
    #[must_use]
    pub fn frontend(message: impl Into<String>) -> Self {
        Self::new(QueryPlanErrorKind::Frontend, message, None)
    }

    /// Error category.
    #[must_use]
    pub const fn kind(&self) -> QueryPlanErrorKind {
        self.kind
    }

    /// Optional source byte offset.
    #[must_use]
    pub const fn offset(&self) -> Option<usize> {
        self.offset
    }
}

impl fmt::Display for QueryPlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(offset) = self.offset {
            write!(formatter, "{} at byte {offset}", self.message)
        } else {
            formatter.write_str(&self.message)
        }
    }
}

impl std::error::Error for QueryPlanError {}

/// Pluggable source-language adapter that produces canonical query plans.
pub trait QueryFrontend: fmt::Debug + Send + Sync {
    /// Lowers a complete source statement.
    fn lower(&self, source: &str, language: &str) -> Result<QueryPlan, QueryPlanError>;
}

/// Built-in SQL common-subset frontend.
#[derive(Clone, Copy, Debug, Default)]
pub struct BuiltInSqlFrontend;

impl QueryFrontend for BuiltInSqlFrontend {
    fn lower(&self, source: &str, language: &str) -> Result<QueryPlan, QueryPlanError> {
        lower_sql(source, language)
    }
}

/// Extensible case-insensitive query-frontend registry.
#[derive(Clone)]
pub struct QueryPlanRegistry {
    frontends: HashMap<String, Arc<dyn QueryFrontend>>,
}

impl QueryPlanRegistry {
    /// Creates a registry containing every built-in SQL profile.
    #[must_use]
    pub fn new() -> Self {
        let sql: Arc<dyn QueryFrontend> = Arc::new(BuiltInSqlFrontend);
        let frontends = SQL_DIALECT_PROFILES
            .iter()
            .map(|profile| (profile.key.to_string(), Arc::clone(&sql)))
            .collect();
        Self { frontends }
    }

    /// Registers or replaces a frontend for a language key.
    pub fn register(
        &mut self,
        language: impl Into<String>,
        frontend: Arc<dyn QueryFrontend>,
    ) -> &mut Self {
        self.frontends
            .insert(language.into().to_ascii_lowercase(), frontend);
        self
    }

    /// Lowers a complete source statement through its registered frontend.
    pub fn lower(&self, source: &str, language: &str) -> Result<QueryPlan, QueryPlanError> {
        self.frontends
            .get(&language.to_ascii_lowercase())
            .ok_or_else(|| unsupported_language(language))?
            .lower(source, language)
    }

    /// Whether an explicit built-in or custom frontend exists.
    #[must_use]
    pub fn is_registered(&self, language: &str) -> bool {
        self.frontends.contains_key(&language.to_ascii_lowercase())
    }
}

impl Default for QueryPlanRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for QueryPlanRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut languages = self.frontends.keys().collect::<Vec<_>>();
        languages.sort_unstable();
        formatter
            .debug_struct("QueryPlanRegistry")
            .field("languages", &languages)
            .finish()
    }
}

/// Parses a supported SQL profile, verifies its complete CST, and lowers it.
pub fn lower_sql(source: &str, language: &str) -> Result<QueryPlan, QueryPlanError> {
    if SqlDialectProfile::lookup(language).is_none() {
        return Err(unsupported_language(language));
    }
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    lower_sql_cst(&network, language)
}

/// Lowers a clean SQL CST, preserving its syntax-link evidence.
pub fn lower_sql_cst(network: &LinkNetwork, language: &str) -> Result<QueryPlan, QueryPlanError> {
    if SqlDialectProfile::lookup(language).is_none() {
        return Err(unsupported_language(language));
    }
    if !network.verify_full_match(None).is_clean() {
        return Err(QueryPlanError::new(
            QueryPlanErrorKind::InvalidConcreteSyntax,
            "SQL CST contains recovery or error links",
            None,
        ));
    }
    let source = network.reconstruct_text();
    if source.trim().is_empty() {
        return Err(QueryPlanError::new(
            QueryPlanErrorKind::Syntax,
            "SQL statement is empty",
            Some(0),
        ));
    }
    let operation = sql::parse(&source)?;
    let syntax = statement_syntax_link(network, language, operation.label()).ok_or_else(|| {
        QueryPlanError::new(
            QueryPlanErrorKind::InvalidConcreteSyntax,
            "SQL lowering requires grammar-backed CST syntax evidence",
            None,
        )
    })?;
    let span = network
        .link(syntax)
        .and_then(|link| link.metadata().span())
        .expect("selected syntax evidence has a source span");
    Ok(QueryPlan::new(
        operation,
        SourceEvidence::new(language, span, Some(syntax)),
    ))
}

fn unsupported_language(language: &str) -> QueryPlanError {
    QueryPlanError::new(
        QueryPlanErrorKind::UnsupportedLanguage,
        format!("no query frontend is registered for `{language}`"),
        None,
    )
}

fn statement_syntax_link(network: &LinkNetwork, language: &str, operation: &str) -> Option<LinkId> {
    network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        .filter(|link| {
            link.metadata()
                .language()
                .is_some_and(|value| value.eq_ignore_ascii_case(language))
        })
        .filter_map(|link| {
            let span = link.metadata().span()?;
            let term = link.metadata().term().unwrap_or_default();
            let score = usize::from(term.eq_ignore_ascii_case(operation)) * 2
                + usize::from(term.to_ascii_lowercase().contains(operation));
            Some((
                score,
                span.byte_range().end() - span.byte_range().start(),
                link.id(),
            ))
        })
        .max_by_key(|(score, length, id)| (*length, *score, id.as_u64()))
        .map(|(_score, _length, id)| id)
}

fn insert_semantic(
    network: &mut LinkNetwork,
    parent: LinkId,
    term: &str,
    definition: Option<&str>,
    links: &mut Vec<LinkId>,
) -> LinkId {
    let mut metadata = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_term(term);
    if let Some(definition) = definition {
        metadata = metadata.with_definition(definition);
    }
    let id = network.insert_link([parent], metadata);
    links.push(id);
    id
}

fn declare_operation(
    network: &mut LinkNetwork,
    parent: LinkId,
    operation: &QueryOperation,
    links: &mut Vec<LinkId>,
) {
    match operation {
        QueryOperation::Select {
            projection,
            source,
            predicate,
            group_by,
            order_by,
            limit,
            offset,
            ..
        } => {
            for item in projection {
                let projection_link = insert_semantic(
                    network,
                    parent,
                    "query-projection",
                    item.alias.as_deref(),
                    links,
                );
                declare_expression(network, projection_link, &item.expression, links);
            }
            if let Some(source) = source {
                declare_source(network, parent, "query-source", source, links);
            }
            if let Some(predicate) = predicate {
                let filter = insert_semantic(network, parent, "query-filter", None, links);
                declare_expression(network, filter, predicate, links);
            }
            for expression in group_by {
                let group = insert_semantic(network, parent, "query-group", None, links);
                declare_expression(network, group, expression, links);
            }
            for sort in order_by {
                let order = insert_semantic(
                    network,
                    parent,
                    "query-order",
                    Some(match sort.direction {
                        SortDirection::Ascending => "ascending",
                        SortDirection::Descending => "descending",
                    }),
                    links,
                );
                declare_expression(network, order, &sort.expression, links);
            }
            if let Some(limit) = limit {
                insert_semantic(
                    network,
                    parent,
                    "query-limit",
                    Some(&limit.to_string()),
                    links,
                );
            }
            if let Some(offset) = offset {
                insert_semantic(
                    network,
                    parent,
                    "query-offset",
                    Some(&offset.to_string()),
                    links,
                );
            }
        }
        QueryOperation::Insert { into, rows, .. } => {
            declare_source(network, parent, "query-target", into, links);
            for row in rows {
                let row_link = insert_semantic(network, parent, "query-insert-row", None, links);
                for expression in row {
                    declare_expression(network, row_link, expression, links);
                }
            }
        }
        QueryOperation::Update {
            table,
            assignments,
            predicate,
        } => {
            declare_source(network, parent, "query-target", table, links);
            for assignment in assignments {
                let assignment_link = insert_semantic(
                    network,
                    parent,
                    "query-assignment",
                    Some(&assignment.column.join(".")),
                    links,
                );
                declare_expression(network, assignment_link, &assignment.value, links);
            }
            if let Some(predicate) = predicate {
                let filter = insert_semantic(network, parent, "query-filter", None, links);
                declare_expression(network, filter, predicate, links);
            }
        }
        QueryOperation::Delete { source, predicate } => {
            declare_source(network, parent, "query-target", source, links);
            if let Some(predicate) = predicate {
                let filter = insert_semantic(network, parent, "query-filter", None, links);
                declare_expression(network, filter, predicate, links);
            }
        }
    }
}

fn declare_source(
    network: &mut LinkNetwork,
    parent: LinkId,
    term: &str,
    source: &QuerySource,
    links: &mut Vec<LinkId>,
) {
    insert_semantic(network, parent, term, Some(&source.path.join(".")), links);
}

fn declare_expression(
    network: &mut LinkNetwork,
    parent: LinkId,
    expression: &QueryExpression,
    links: &mut Vec<LinkId>,
) {
    let (term, definition) = match expression {
        QueryExpression::Column { path } => ("query-column".to_string(), Some(path.join("."))),
        QueryExpression::Literal { value } => {
            ("query-literal".to_string(), Some(format!("{value:?}")))
        }
        QueryExpression::Parameter { name } => ("query-parameter".to_string(), Some(name.clone())),
        QueryExpression::Wildcard => ("query-wildcard".to_string(), None),
        QueryExpression::Unary { operator, .. } => (
            format!("query-unary:{operator:?}").to_ascii_lowercase(),
            None,
        ),
        QueryExpression::Binary { operator, .. } => (
            format!("query-binary:{operator:?}").to_ascii_lowercase(),
            None,
        ),
        QueryExpression::Aggregate { function, .. } => {
            (format!("query-aggregate:{}", function.label()), None)
        }
        QueryExpression::Function { name, .. } => {
            ("query-function".to_string(), Some(name.clone()))
        }
        QueryExpression::Extension {
            namespace, name, ..
        } => (
            "query-extension".to_string(),
            Some(format!("{namespace}:{name}")),
        ),
    };
    let expression_link = insert_semantic(network, parent, &term, definition.as_deref(), links);
    match expression {
        QueryExpression::Unary { operand, .. } => {
            declare_expression(network, expression_link, operand, links);
        }
        QueryExpression::Binary { left, right, .. } => {
            declare_expression(network, expression_link, left, links);
            declare_expression(network, expression_link, right, links);
        }
        QueryExpression::Aggregate { expression, .. } => {
            declare_expression(network, expression_link, expression, links);
        }
        QueryExpression::Function { arguments, .. }
        | QueryExpression::Extension { arguments, .. } => {
            for argument in arguments {
                declare_expression(network, expression_link, argument, links);
            }
        }
        QueryExpression::Column { .. }
        | QueryExpression::Literal { .. }
        | QueryExpression::Parameter { .. }
        | QueryExpression::Wildcard => {}
    }
}
