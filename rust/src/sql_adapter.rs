//! Registry-driven lowering from validated SQL statements to shared query plans.

#[path = "query_plan/sql.rs"]
mod parser;

use std::collections::BTreeMap;
use std::fmt;

use serde_json::Value;

use crate::configuration::ParseConfiguration;
use crate::link_network::{LinkNetwork, LinkType};
use crate::query_plan::{
    attach_plan_links, LoweredQueryPlan, QueryAggregate, QueryAggregateFunction,
    QueryComparisonOperator, QueryFilter, QueryOperation as CanonicalOperation, QueryOrder,
    QueryPlan, QuerySortDirection, QuerySourceEvidence, QueryValue as CanonicalValue,
};
use crate::source::SourceSpan;

/// SQL profiles whose common subset is normalized by the built-in adapter.
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

/// One explicit SQL relation and field mapping.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqlRelationMapping {
    source_relation: String,
    resource: String,
    fields: BTreeMap<String, String>,
}

impl SqlRelationMapping {
    /// Creates a relation mapping. Fields remain unsupported until registered.
    #[must_use]
    pub fn new(source_relation: impl Into<String>, resource: impl Into<String>) -> Self {
        Self {
            source_relation: source_relation.into(),
            resource: resource.into(),
            fields: BTreeMap::new(),
        }
    }

    /// Adds a source-column to canonical-field mapping.
    #[must_use]
    pub fn with_field(
        mut self,
        source_name: impl Into<String>,
        canonical_field: impl Into<String>,
    ) -> Self {
        self.fields
            .insert(source_name.into(), canonical_field.into());
        self
    }

    /// SQL relation name, optionally schema-qualified.
    #[must_use]
    pub fn source_relation(&self) -> &str {
        &self.source_relation
    }

    /// Canonical resource name.
    #[must_use]
    pub fn resource(&self) -> &str {
        &self.resource
    }

    fn mapped_field(&self, source: &str) -> Result<String, SqlAdapterError> {
        let matches = self
            .fields
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case(source))
            .map(|(_, canonical)| canonical)
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [canonical] => Ok((*canonical).clone()),
            [] => Err(SqlAdapterError::semantic(format!(
                "unmapped SQL field {source:?} for relation {:?}",
                self.source_relation
            ))),
            _ => Err(SqlAdapterError::registry(format!(
                "ambiguous SQL field mapping {source:?} for relation {:?}",
                self.source_relation
            ))),
        }
    }
}

/// Explicit, fail-closed schema registry used by SQL lowering.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SqlSchemaRegistry {
    relations: BTreeMap<String, SqlRelationMapping>,
}

impl SqlSchemaRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            relations: BTreeMap::new(),
        }
    }

    /// Registers one relation mapping.
    pub fn register_relation(
        &mut self,
        mapping: SqlRelationMapping,
    ) -> Result<(), SqlAdapterError> {
        validate_mapping(&mapping)?;
        if self
            .relations
            .keys()
            .any(|name| name.eq_ignore_ascii_case(&mapping.source_relation))
        {
            return Err(SqlAdapterError::registry(format!(
                "duplicate or case-ambiguous SQL relation mapping {:?}",
                mapping.source_relation
            )));
        }
        self.relations
            .insert(mapping.source_relation.clone(), mapping);
        Ok(())
    }

    /// Loads the documented JSON registry shape used by shared parity fixtures.
    pub fn from_json(value: &Value) -> Result<Self, SqlAdapterError> {
        let relations = value
            .get("relations")
            .and_then(Value::as_array)
            .ok_or_else(|| SqlAdapterError::registry("registry.relations must be an array"))?;
        let mut registry = Self::new();
        for relation in relations {
            let mut mapping = SqlRelationMapping::new(
                required_string(relation, "sourceRelation")?,
                required_string(relation, "resource")?,
            );
            let fields = relation
                .get("fields")
                .and_then(Value::as_object)
                .ok_or_else(|| SqlAdapterError::registry("relation.fields must be an object"))?;
            for (source, canonical) in fields {
                let canonical = canonical.as_str().ok_or_else(|| {
                    SqlAdapterError::registry("canonical SQL fields must be strings")
                })?;
                mapping = mapping.with_field(source, canonical);
            }
            registry.register_relation(mapping)?;
        }
        Ok(registry)
    }

    fn relation(&self, path: &[String]) -> Result<&SqlRelationMapping, SqlAdapterError> {
        let source = path.join(".");
        let matches = self
            .relations
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case(&source))
            .map(|(_, mapping)| mapping)
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [mapping] => Ok(*mapping),
            [] => Err(SqlAdapterError::semantic(format!(
                "unmapped SQL relation {source:?}"
            ))),
            _ => Err(SqlAdapterError::registry(format!(
                "ambiguous SQL relation mapping {source:?}"
            ))),
        }
    }
}

/// Error category for fail-closed SQL lowering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqlAdapterErrorKind {
    UnsupportedLanguage,
    InvalidConcreteSyntax,
    Syntax,
    Semantic,
    Registry,
}

/// Fail-closed SQL adapter error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SqlAdapterError {
    kind: SqlAdapterErrorKind,
    message: String,
    offset: Option<usize>,
}

impl SqlAdapterError {
    fn new(kind: SqlAdapterErrorKind, message: impl Into<String>, offset: Option<usize>) -> Self {
        Self {
            kind,
            message: message.into(),
            offset,
        }
    }

    fn semantic(message: impl Into<String>) -> Self {
        Self::new(SqlAdapterErrorKind::Semantic, message, None)
    }

    fn registry(message: impl Into<String>) -> Self {
        Self::new(SqlAdapterErrorKind::Registry, message, None)
    }

    /// Error category.
    #[must_use]
    pub const fn kind(&self) -> SqlAdapterErrorKind {
        self.kind
    }

    /// Optional source byte offset.
    #[must_use]
    pub const fn offset(&self) -> Option<usize> {
        self.offset
    }
}

impl fmt::Display for SqlAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(offset) = self.offset {
            write!(formatter, "{} at byte {offset}", self.message)
        } else {
            formatter.write_str(&self.message)
        }
    }
}

impl std::error::Error for SqlAdapterError {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QuerySource {
    path: Vec<String>,
    alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Projection {
    expression: QueryExpression,
    alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Assignment {
    column: Vec<String>,
    value: QueryExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SortExpression {
    expression: QueryExpression,
    direction: SortDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryValue {
    Null,
    Boolean { value: bool },
    Number { value: String },
    String { value: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AggregateFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    VariancePopulation,
    StandardDeviationPopulation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnaryOperator {
    Not,
    Negate,
    Positive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BinaryOperator {
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryExpression {
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryOperation {
    Select {
        distinct: bool,
        projection: Vec<Projection>,
        source: Option<QuerySource>,
        predicate: Option<QueryExpression>,
        group_by: Vec<QueryExpression>,
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
        predicate: Option<QueryExpression>,
    },
    Delete {
        source: QuerySource,
        predicate: Option<QueryExpression>,
    },
}

/// Parses, validates, and lowers exactly one SQL statement and its provenance.
pub fn lower_sql(
    source: &str,
    language: &str,
    registry: &SqlSchemaRegistry,
) -> Result<LoweredQueryPlan, SqlAdapterError> {
    ensure_profile(language)?;
    let mut network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let plan = lower_sql_cst(&network, language, registry)?;
    let root_link = attach_plan_links(&mut network, &plan, language);
    Ok(LoweredQueryPlan::new(plan, network, root_link))
}

/// Lowers one complete, clean SQL CST into the shared executable IR.
pub fn lower_sql_cst(
    network: &LinkNetwork,
    language: &str,
    registry: &SqlSchemaRegistry,
) -> Result<QueryPlan, SqlAdapterError> {
    ensure_profile(language)?;
    if !network.verify_full_match(None).is_clean() {
        return Err(SqlAdapterError::new(
            SqlAdapterErrorKind::InvalidConcreteSyntax,
            "SQL CST validation failed; semantic lowering was not attempted",
            None,
        ));
    }
    let source = network.reconstruct_text();
    if source.trim().is_empty() {
        return Err(SqlAdapterError::new(
            SqlAdapterErrorKind::Syntax,
            "SQL statement is empty",
            Some(0),
        ));
    }
    let operation = parser::parse(&source)?;
    let operation_label = match &operation {
        QueryOperation::Select { .. } => "select",
        QueryOperation::Insert { .. } => "insert",
        QueryOperation::Update { .. } => "update",
        QueryOperation::Delete { .. } => "delete",
    };
    let mut plan = lower_operation(operation, registry)?;
    let span = statement_span(network, language, operation_label).ok_or_else(|| {
        SqlAdapterError::new(
            SqlAdapterErrorKind::InvalidConcreteSyntax,
            "SQL lowering requires grammar-backed CST syntax evidence",
            None,
        )
    })?;
    plan.add_source_evidence(QuerySourceEvidence::new(
        format!("statement:{language}"),
        span,
    ));
    Ok(plan)
}

fn lower_operation(
    operation: QueryOperation,
    registry: &SqlSchemaRegistry,
) -> Result<QueryPlan, SqlAdapterError> {
    match operation {
        QueryOperation::Select {
            distinct,
            projection,
            source,
            predicate,
            group_by,
            order_by,
            limit,
            offset,
        } => {
            if distinct {
                return Err(SqlAdapterError::semantic(
                    "SELECT DISTINCT requires an explicit query-plan extension",
                ));
            }
            let source = source.ok_or_else(|| {
                SqlAdapterError::semantic("SELECT without FROM has no canonical resource")
            })?;
            let mapping = registry.relation(&source.path)?;
            let mut plan = QueryPlan::new(CanonicalOperation::Select, mapping.resource());
            for item in projection {
                lower_projection(item, mapping, &source, &mut plan)?;
            }
            if plan.projection.is_empty() && plan.aggregates.is_empty() {
                return Err(SqlAdapterError::semantic(
                    "SELECT requires a mapped projection or aggregate",
                ));
            }
            plan.filter = predicate
                .as_ref()
                .map(|value| lower_filter(value, mapping, &source))
                .transpose()?;
            plan.group_by = group_by
                .iter()
                .map(|value| mapped_column(value, mapping, &source))
                .collect::<Result<Vec<_>, _>>()?;
            plan.order = order_by
                .iter()
                .map(|order| {
                    Ok(QueryOrder::new(
                        mapped_column(&order.expression, mapping, &source)?,
                        match order.direction {
                            SortDirection::Ascending => QuerySortDirection::Ascending,
                            SortDirection::Descending => QuerySortDirection::Descending,
                        },
                    ))
                })
                .collect::<Result<Vec<_>, SqlAdapterError>>()?;
            plan.set_pagination(limit, offset);
            Ok(plan)
        }
        QueryOperation::Insert {
            into,
            columns,
            rows,
        } => {
            let mapping = registry.relation(&into.path)?;
            let [row] = rows.as_slice() else {
                return Err(SqlAdapterError::semantic(
                    "multi-row INSERT requires an explicit query-plan extension",
                ));
            };
            if columns.is_empty() {
                return Err(SqlAdapterError::semantic(
                    "INSERT requires an explicit mapped column list",
                ));
            }
            let mut plan = QueryPlan::new(CanonicalOperation::Insert, mapping.resource());
            for (column, value) in columns.iter().zip(row) {
                plan.set_mutation_value(mapping.mapped_field(column)?, lower_value(value)?);
            }
            Ok(plan)
        }
        QueryOperation::Update {
            table,
            assignments,
            predicate,
        } => {
            let mapping = registry.relation(&table.path)?;
            let mut plan = QueryPlan::new(CanonicalOperation::Update, mapping.resource());
            for assignment in assignments {
                let field = mapped_path(&assignment.column, mapping, &table)?;
                plan.set_mutation_value(field, lower_value(&assignment.value)?);
            }
            plan.filter = predicate
                .as_ref()
                .map(|value| lower_filter(value, mapping, &table))
                .transpose()?;
            Ok(plan)
        }
        QueryOperation::Delete { source, predicate } => {
            let mapping = registry.relation(&source.path)?;
            let mut plan = QueryPlan::new(CanonicalOperation::Delete, mapping.resource());
            plan.filter = predicate
                .as_ref()
                .map(|value| lower_filter(value, mapping, &source))
                .transpose()?;
            Ok(plan)
        }
    }
}

fn lower_projection(
    projection: Projection,
    mapping: &SqlRelationMapping,
    source: &QuerySource,
    plan: &mut QueryPlan,
) -> Result<(), SqlAdapterError> {
    match projection.expression {
        QueryExpression::Column { path } => {
            if projection.alias.is_some() {
                return Err(SqlAdapterError::semantic(
                    "non-aggregate projection aliases are not represented by the query plan",
                ));
            }
            let field = mapped_path(&path, mapping, source)?;
            if !plan.projection.contains(&field) {
                plan.add_projection(field);
            }
        }
        QueryExpression::Aggregate {
            function,
            expression,
            distinct,
        } => {
            if distinct {
                return Err(SqlAdapterError::semantic(
                    "DISTINCT aggregates require an explicit query-plan extension",
                ));
            }
            let field = match expression.as_ref() {
                QueryExpression::Wildcard if function == AggregateFunction::Count => None,
                expression => Some(mapped_column(expression, mapping, source)?),
            };
            if function != AggregateFunction::Count && field.is_none() {
                return Err(SqlAdapterError::semantic(
                    "non-count aggregates require a mapped field",
                ));
            }
            plan.add_aggregate(QueryAggregate::new(
                aggregate_function(function),
                field,
                projection.alias,
            ));
        }
        _ => {
            return Err(SqlAdapterError::semantic(
                "SQL projection expression is outside the shared query-plan subset",
            ));
        }
    }
    Ok(())
}

fn lower_filter(
    expression: &QueryExpression,
    mapping: &SqlRelationMapping,
    source: &QuerySource,
) -> Result<QueryFilter, SqlAdapterError> {
    match expression {
        QueryExpression::Unary {
            operator: UnaryOperator::Not,
            operand,
        } => Ok(QueryFilter::Not(Box::new(lower_filter(
            operand, mapping, source,
        )?))),
        QueryExpression::Binary {
            operator: BinaryOperator::And,
            left,
            right,
        } => Ok(QueryFilter::And(vec![
            lower_filter(left, mapping, source)?,
            lower_filter(right, mapping, source)?,
        ])),
        QueryExpression::Binary {
            operator: BinaryOperator::Or,
            left,
            right,
        } => Ok(QueryFilter::Or(vec![
            lower_filter(left, mapping, source)?,
            lower_filter(right, mapping, source)?,
        ])),
        QueryExpression::Binary {
            operator,
            left,
            right,
        } => {
            let field = mapped_column(left, mapping, source)?;
            let (operator, value) = comparison(*operator, right)?;
            Ok(QueryFilter::Compare {
                field,
                operator,
                value,
            })
        }
        _ => Err(SqlAdapterError::semantic(
            "SQL predicate is outside the shared query-plan subset",
        )),
    }
}

fn comparison(
    operator: BinaryOperator,
    right: &QueryExpression,
) -> Result<(QueryComparisonOperator, CanonicalValue), SqlAdapterError> {
    let canonical = match operator {
        BinaryOperator::Equal => QueryComparisonOperator::Equal,
        BinaryOperator::NotEqual => QueryComparisonOperator::NotEqual,
        BinaryOperator::LessThan => QueryComparisonOperator::LessThan,
        BinaryOperator::LessThanOrEqual => QueryComparisonOperator::LessThanOrEqual,
        BinaryOperator::GreaterThan => QueryComparisonOperator::GreaterThan,
        BinaryOperator::GreaterThanOrEqual => QueryComparisonOperator::GreaterThanOrEqual,
        BinaryOperator::Like => QueryComparisonOperator::Like,
        BinaryOperator::Is | BinaryOperator::IsNot => {
            if !matches!(
                right,
                QueryExpression::Literal {
                    value: QueryValue::Null
                }
            ) {
                return Err(SqlAdapterError::semantic(
                    "only IS NULL and IS NOT NULL are in the shared query-plan subset",
                ));
            }
            return Ok((
                QueryComparisonOperator::IsNull,
                CanonicalValue::Boolean(operator == BinaryOperator::Is),
            ));
        }
        BinaryOperator::NotLike
        | BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::And
        | BinaryOperator::Or => {
            return Err(SqlAdapterError::semantic(
                "SQL comparison requires an explicit query-plan extension",
            ));
        }
    };
    Ok((canonical, lower_value(right)?))
}

fn mapped_column(
    expression: &QueryExpression,
    mapping: &SqlRelationMapping,
    source: &QuerySource,
) -> Result<String, SqlAdapterError> {
    let QueryExpression::Column { path } = expression else {
        return Err(SqlAdapterError::semantic(
            "query-plan fields must be direct mapped SQL columns",
        ));
    };
    mapped_path(path, mapping, source)
}

fn mapped_path(
    path: &[String],
    mapping: &SqlRelationMapping,
    source: &QuerySource,
) -> Result<String, SqlAdapterError> {
    let Some(field) = path.last() else {
        return Err(SqlAdapterError::semantic("SQL column path is empty"));
    };
    if path.len() > 1 {
        let qualifier = path[..path.len() - 1].join(".");
        let source_name = source.path.join(".");
        let source_tail = source.path.last().map(String::as_str).unwrap_or_default();
        let qualifier_matches = qualifier.eq_ignore_ascii_case(&source_name)
            || qualifier.eq_ignore_ascii_case(source_tail)
            || source
                .alias
                .as_deref()
                .is_some_and(|alias| qualifier.eq_ignore_ascii_case(alias));
        if !qualifier_matches {
            return Err(SqlAdapterError::semantic(format!(
                "SQL column qualifier {qualifier:?} does not identify the mapped relation"
            )));
        }
    }
    mapping.mapped_field(field)
}

fn lower_value(expression: &QueryExpression) -> Result<CanonicalValue, SqlAdapterError> {
    match expression {
        QueryExpression::Literal { value } => match value {
            QueryValue::Null => Ok(CanonicalValue::Null),
            QueryValue::Boolean { value } => Ok(CanonicalValue::Boolean(*value)),
            QueryValue::Number { value } => parse_number(value),
            QueryValue::String { value } => Ok(CanonicalValue::String(value.clone())),
        },
        QueryExpression::Unary {
            operator: UnaryOperator::Negate,
            operand,
        } => match lower_value(operand)? {
            CanonicalValue::Integer(value) => {
                safe_integer(value.checked_neg().ok_or_else(|| {
                    SqlAdapterError::semantic("SQL integer is outside the supported range")
                })?)
            }
            CanonicalValue::Float(value) => Ok(CanonicalValue::Float(-value)),
            _ => Err(SqlAdapterError::semantic(
                "unary minus requires a numeric SQL literal",
            )),
        },
        QueryExpression::Unary {
            operator: UnaryOperator::Positive,
            operand,
        } => lower_value(operand),
        _ => Err(SqlAdapterError::semantic(
            "query-plan values must be bounded SQL literals",
        )),
    }
}

fn parse_number(value: &str) -> Result<CanonicalValue, SqlAdapterError> {
    if value.contains('.') {
        let value = value
            .parse::<f64>()
            .map_err(|_| SqlAdapterError::semantic("invalid SQL floating-point literal"))?;
        if !value.is_finite() {
            return Err(SqlAdapterError::semantic(
                "SQL floating-point values must be finite",
            ));
        }
        Ok(CanonicalValue::Float(value))
    } else {
        let value = value
            .parse::<i64>()
            .map_err(|_| SqlAdapterError::semantic("SQL integer is outside the supported range"))?;
        safe_integer(value)
    }
}

fn safe_integer(value: i64) -> Result<CanonicalValue, SqlAdapterError> {
    const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
    if (-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&value) {
        Ok(CanonicalValue::Integer(value))
    } else {
        Err(SqlAdapterError::semantic(
            "SQL integer is outside the cross-language safe range",
        ))
    }
}

const fn aggregate_function(function: AggregateFunction) -> QueryAggregateFunction {
    match function {
        AggregateFunction::Count => QueryAggregateFunction::Count,
        AggregateFunction::Sum => QueryAggregateFunction::Sum,
        AggregateFunction::Avg => QueryAggregateFunction::Average,
        AggregateFunction::Min => QueryAggregateFunction::Minimum,
        AggregateFunction::Max => QueryAggregateFunction::Maximum,
        AggregateFunction::VariancePopulation => QueryAggregateFunction::PopulationVariance,
        AggregateFunction::StandardDeviationPopulation => {
            QueryAggregateFunction::PopulationStandardDeviation
        }
    }
}

fn ensure_profile(language: &str) -> Result<(), SqlAdapterError> {
    if SqlDialectProfile::lookup(language).is_some() {
        Ok(())
    } else {
        Err(SqlAdapterError::new(
            SqlAdapterErrorKind::UnsupportedLanguage,
            format!("unsupported SQL profile {language:?}"),
            None,
        ))
    }
}

fn statement_span(network: &LinkNetwork, language: &str, operation: &str) -> Option<SourceSpan> {
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
            Some((span, score))
        })
        .max_by_key(|(span, score)| (span.byte_range().end() - span.byte_range().start(), *score))
        .map(|(span, _)| span)
}

fn validate_mapping(mapping: &SqlRelationMapping) -> Result<(), SqlAdapterError> {
    if mapping.source_relation.trim().is_empty() || mapping.resource.trim().is_empty() {
        return Err(SqlAdapterError::registry(
            "SQL relation and canonical resource names must not be empty",
        ));
    }
    if mapping.fields.is_empty() {
        return Err(SqlAdapterError::registry(
            "SQL relation mappings require at least one explicit field",
        ));
    }
    let names = mapping.fields.keys().collect::<Vec<_>>();
    for (index, left) in names.iter().enumerate() {
        if left.trim().is_empty() || mapping.fields[*left].trim().is_empty() {
            return Err(SqlAdapterError::registry(
                "SQL field mapping names must not be empty",
            ));
        }
        if names[index + 1..]
            .iter()
            .any(|right| left.eq_ignore_ascii_case(right))
        {
            return Err(SqlAdapterError::registry(format!(
                "case-ambiguous SQL field mapping {left:?}"
            )));
        }
    }
    Ok(())
}

fn required_string<'a>(value: &'a Value, name: &str) -> Result<&'a str, SqlAdapterError> {
    value.get(name).and_then(Value::as_str).ok_or_else(|| {
        SqlAdapterError::registry(format!("registry field {name:?} must be a string"))
    })
}
