//! Language-neutral executable query-plan concepts shared by query frontends.

use std::collections::BTreeMap;

use serde_json::{json, Map, Number, Value};

use crate::source::SourceSpan;

/// Stable schema version emitted by [`QueryPlan::canonical_json`].
pub const QUERY_PLAN_VERSION: u8 = 1;

/// Canonical executable operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryOperation {
    /// Read matching resources.
    Select,
    /// Create a resource.
    Insert,
    /// Modify matching resources.
    Update,
    /// Remove matching resources.
    Delete,
}

impl QueryOperation {
    /// Canonical lower-case label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }

    /// Parses a canonical operation label.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "select" => Some(Self::Select),
            "insert" => Some(Self::Insert),
            "update" => Some(Self::Update),
            "delete" => Some(Self::Delete),
            _ => None,
        }
    }
}

/// Canonical comparison operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryComparisonOperator {
    /// Equal.
    Equal,
    /// Not equal.
    NotEqual,
    /// Less than.
    LessThan,
    /// Less than or equal.
    LessThanOrEqual,
    /// Greater than.
    GreaterThan,
    /// Greater than or equal.
    GreaterThanOrEqual,
    /// Membership in a list.
    In,
    /// Non-membership in a list.
    NotIn,
    /// String/pattern match.
    Like,
    /// Null predicate; the boolean value selects `IS NULL` or `IS NOT NULL`.
    IsNull,
}

impl QueryComparisonOperator {
    /// Canonical label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "eq",
            Self::NotEqual => "neq",
            Self::LessThan => "lt",
            Self::LessThanOrEqual => "lte",
            Self::GreaterThan => "gt",
            Self::GreaterThanOrEqual => "gte",
            Self::In => "in",
            Self::NotIn => "not-in",
            Self::Like => "like",
            Self::IsNull => "is-null",
        }
    }

    /// Maps the adapter's standard GraphQL filter key to a canonical operator.
    #[must_use]
    pub fn from_graphql_key(value: &str) -> Option<Self> {
        match value {
            "eq" => Some(Self::Equal),
            "neq" | "ne" => Some(Self::NotEqual),
            "lt" => Some(Self::LessThan),
            "lte" => Some(Self::LessThanOrEqual),
            "gt" => Some(Self::GreaterThan),
            "gte" => Some(Self::GreaterThanOrEqual),
            "in" => Some(Self::In),
            "notIn" => Some(Self::NotIn),
            "like" => Some(Self::Like),
            "isNull" => Some(Self::IsNull),
            _ => None,
        }
    }
}

/// Scalar and composite literal used by filters and mutations.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryValue {
    /// GraphQL/JSON null.
    Null,
    /// Boolean literal.
    Boolean(bool),
    /// Integer literal.
    Integer(i64),
    /// Finite floating-point literal.
    Float(f64),
    /// String or enum literal. Canonical plans intentionally erase that
    /// source-syntax distinction.
    String(String),
    /// Ordered list literal.
    List(Vec<Self>),
    /// Deterministically ordered object literal.
    Object(BTreeMap<String, Self>),
}

impl QueryValue {
    /// Converts to the JSON value used in canonical plans.
    #[must_use]
    pub fn to_json(&self) -> Value {
        match self {
            Self::Null => Value::Null,
            Self::Boolean(value) => Value::Bool(*value),
            Self::Integer(value) => Value::Number(Number::from(*value)),
            Self::Float(value) => Value::Number(
                Number::from_f64(*value).expect("query plan floating-point values must be finite"),
            ),
            Self::String(value) => Value::String(value.clone()),
            Self::List(values) => Value::Array(values.iter().map(Self::to_json).collect()),
            Self::Object(values) => Value::Object(
                values
                    .iter()
                    .map(|(key, value)| (key.clone(), value.to_json()))
                    .collect(),
            ),
        }
    }
}

/// Canonical boolean filter tree.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryFilter {
    /// Field comparison.
    Compare {
        /// Canonical domain field.
        field: String,
        /// Comparison operator.
        operator: QueryComparisonOperator,
        /// Right-hand literal.
        value: QueryValue,
    },
    /// All children must match.
    And(Vec<Self>),
    /// At least one child must match.
    Or(Vec<Self>),
    /// Child must not match.
    Not(Box<Self>),
}

impl QueryFilter {
    fn to_json(&self) -> Value {
        match self {
            Self::Compare {
                field,
                operator,
                value,
            } => json!({
                "compare": {
                    "field": field,
                    "operator": operator.as_str(),
                    "value": value.to_json(),
                }
            }),
            Self::And(children) => {
                json!({"and": children.iter().map(Self::to_json).collect::<Vec<_>>()})
            }
            Self::Or(children) => {
                json!({"or": children.iter().map(Self::to_json).collect::<Vec<_>>()})
            }
            Self::Not(child) => json!({"not": child.to_json()}),
        }
    }
}

/// Sort direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuerySortDirection {
    /// Ascending.
    Ascending,
    /// Descending.
    Descending,
}

impl QuerySortDirection {
    /// Canonical label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ascending => "asc",
            Self::Descending => "desc",
        }
    }
}

/// Canonical ordering entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryOrder {
    pub(crate) field: String,
    pub(crate) direction: QuerySortDirection,
}

impl QueryOrder {
    /// Creates an ordering entry for a canonical field.
    #[must_use]
    pub fn new(field: impl Into<String>, direction: QuerySortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }

    /// Canonical field.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Sort direction.
    #[must_use]
    pub const fn direction(&self) -> QuerySortDirection {
        self.direction
    }
}

/// Supported common aggregate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryAggregateFunction {
    /// Row/value count.
    Count,
    /// Sum.
    Sum,
    /// Arithmetic mean.
    Average,
    /// Minimum.
    Minimum,
    /// Maximum.
    Maximum,
    /// Population variance.
    PopulationVariance,
    /// Population standard deviation.
    PopulationStandardDeviation,
}

impl QueryAggregateFunction {
    /// Canonical label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Sum => "sum",
            Self::Average => "avg",
            Self::Minimum => "min",
            Self::Maximum => "max",
            Self::PopulationVariance => "variance-population",
            Self::PopulationStandardDeviation => "stddev-population",
        }
    }

    /// Parses a canonical aggregate label.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "count" => Some(Self::Count),
            "sum" => Some(Self::Sum),
            "avg" => Some(Self::Average),
            "min" => Some(Self::Minimum),
            "max" => Some(Self::Maximum),
            "variance-population" => Some(Self::PopulationVariance),
            "stddev-population" => Some(Self::PopulationStandardDeviation),
            _ => None,
        }
    }
}

/// Aggregate projection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryAggregate {
    pub(crate) function: QueryAggregateFunction,
    pub(crate) field: Option<String>,
    pub(crate) alias: Option<String>,
}

impl QueryAggregate {
    /// Creates an aggregate projection.
    #[must_use]
    pub const fn new(
        function: QueryAggregateFunction,
        field: Option<String>,
        alias: Option<String>,
    ) -> Self {
        Self {
            function,
            field,
            alias,
        }
    }

    /// Aggregate function.
    #[must_use]
    pub const fn function(&self) -> QueryAggregateFunction {
        self.function
    }

    /// Canonical input field, absent for `COUNT(*)`-style operations.
    #[must_use]
    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    /// Result alias, when supplied by the frontend.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }
}

/// Source evidence attached to a canonical plan element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuerySourceEvidence {
    role: String,
    span: SourceSpan,
}

impl QuerySourceEvidence {
    /// Creates evidence connecting a semantic plan role to an exact source range.
    #[must_use]
    pub fn new(role: impl Into<String>, span: SourceSpan) -> Self {
        Self {
            role: role.into(),
            span,
        }
    }

    /// Semantic role evidenced by the source range.
    #[must_use]
    pub fn role(&self) -> &str {
        &self.role
    }

    /// Exact source range.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }
}

/// Language-neutral executable query plan.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryPlan {
    pub(crate) operation: QueryOperation,
    pub(crate) resource: String,
    pub(crate) projection: Vec<String>,
    pub(crate) filter: Option<QueryFilter>,
    pub(crate) order: Vec<QueryOrder>,
    pub(crate) limit: Option<u64>,
    pub(crate) offset: Option<u64>,
    pub(crate) group_by: Vec<String>,
    pub(crate) aggregates: Vec<QueryAggregate>,
    pub(crate) mutation: BTreeMap<String, QueryValue>,
    pub(crate) source_evidence: Vec<QuerySourceEvidence>,
}

impl QueryPlan {
    /// Starts a plan for `operation` over the canonical `resource` name.
    #[must_use]
    pub fn new(operation: QueryOperation, resource: impl Into<String>) -> Self {
        Self {
            operation,
            resource: resource.into(),
            projection: Vec::new(),
            filter: None,
            order: Vec::new(),
            limit: None,
            offset: None,
            group_by: Vec::new(),
            aggregates: Vec::new(),
            mutation: BTreeMap::new(),
            source_evidence: Vec::new(),
        }
    }

    /// Appends a canonical field to the response projection.
    pub fn add_projection(&mut self, field: impl Into<String>) {
        self.projection.push(field.into());
    }

    /// Replaces the boolean filter.
    pub fn set_filter(&mut self, filter: QueryFilter) {
        self.filter = Some(filter);
    }

    /// Appends an ordering entry.
    pub fn add_order(&mut self, order: QueryOrder) {
        self.order.push(order);
    }

    /// Sets result pagination.
    pub const fn set_pagination(&mut self, limit: Option<u64>, offset: Option<u64>) {
        self.limit = limit;
        self.offset = offset;
    }

    /// Appends a canonical grouping field.
    pub fn add_group_by(&mut self, field: impl Into<String>) {
        self.group_by.push(field.into());
    }

    /// Appends an aggregate projection.
    pub fn add_aggregate(&mut self, aggregate: QueryAggregate) {
        self.aggregates.push(aggregate);
    }

    /// Sets a canonical mutation assignment.
    pub fn set_mutation_value(&mut self, field: impl Into<String>, value: QueryValue) {
        self.mutation.insert(field.into(), value);
    }

    /// Attaches source evidence to the plan.
    pub fn add_source_evidence(&mut self, evidence: QuerySourceEvidence) {
        self.source_evidence.push(evidence);
    }

    /// Operation.
    #[must_use]
    pub const fn operation(&self) -> QueryOperation {
        self.operation
    }

    /// Canonical resource name.
    #[must_use]
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Canonical projected fields in response order.
    #[must_use]
    pub fn projection(&self) -> &[String] {
        &self.projection
    }

    /// Boolean filter.
    #[must_use]
    pub const fn filter(&self) -> Option<&QueryFilter> {
        self.filter.as_ref()
    }

    /// Ordering entries.
    #[must_use]
    pub fn order(&self) -> &[QueryOrder] {
        &self.order
    }

    /// Maximum result count.
    #[must_use]
    pub const fn limit(&self) -> Option<u64> {
        self.limit
    }

    /// Result offset.
    #[must_use]
    pub const fn offset(&self) -> Option<u64> {
        self.offset
    }

    /// Canonical grouping fields.
    #[must_use]
    pub fn group_by(&self) -> &[String] {
        &self.group_by
    }

    /// Aggregate projections.
    #[must_use]
    pub fn aggregates(&self) -> &[QueryAggregate] {
        &self.aggregates
    }

    /// Canonical mutation assignments.
    #[must_use]
    pub const fn mutation(&self) -> &BTreeMap<String, QueryValue> {
        &self.mutation
    }

    /// Source ranges retained by the frontend adapter.
    #[must_use]
    pub fn source_evidence(&self) -> &[QuerySourceEvidence] {
        &self.source_evidence
    }

    /// Returns the provenance-free canonical JSON value used to compare plans
    /// produced by different query languages.
    #[must_use]
    pub fn canonical_value(&self) -> Value {
        let order = self
            .order
            .iter()
            .map(|entry| {
                json!({
                    "field": entry.field,
                    "direction": entry.direction.as_str(),
                })
            })
            .collect::<Vec<_>>();
        let aggregates = self
            .aggregates
            .iter()
            .map(|aggregate| {
                json!({
                    "function": aggregate.function.as_str(),
                    "field": aggregate.field,
                    "alias": aggregate.alias,
                })
            })
            .collect::<Vec<_>>();
        let mutation = self
            .mutation
            .iter()
            .map(|(field, value)| (field.clone(), value.to_json()))
            .collect::<Map<_, _>>();

        json!({
            "version": QUERY_PLAN_VERSION,
            "operation": self.operation.as_str(),
            "resource": self.resource,
            "projection": self.projection,
            "filter": self.filter.as_ref().map(QueryFilter::to_json),
            "order": order,
            "limit": self.limit,
            "offset": self.offset,
            "groupBy": self.group_by,
            "aggregates": aggregates,
            "mutation": mutation,
        })
    }

    /// Deterministic provenance-free serialization for cross-frontend
    /// conformance checks.
    #[must_use]
    pub fn canonical_json(&self) -> String {
        serde_json::to_string(&self.canonical_value()).expect("query plan values are serializable")
    }
}
