use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde_json::Value;

use crate::query_plan::{QueryAggregateFunction, QueryOperation};

/// GraphQL operation family used as a registry key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphQlOperationType {
    /// Read operation (including GraphQL's anonymous shorthand form).
    Query,
    /// Mutation operation.
    Mutation,
}

impl GraphQlOperationType {
    /// Canonical registry label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Mutation => "mutation",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "query" => Some(Self::Query),
            "mutation" => Some(Self::Mutation),
            _ => None,
        }
    }
}

/// Semantic role assigned to a GraphQL root-field argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphQlArgumentRole {
    /// Boolean filter object.
    Filter,
    /// Ordering object or list.
    Order,
    /// Non-negative result limit.
    Limit,
    /// Non-negative result offset.
    Offset,
    /// Grouping field or list of fields.
    Group,
    /// Mutation assignment object.
    MutationInput,
}

impl GraphQlArgumentRole {
    /// Canonical registry label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Filter => "filter",
            Self::Order => "order",
            Self::Limit => "limit",
            Self::Offset => "offset",
            Self::Group => "group",
            Self::MutationInput => "mutation-input",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "filter" => Some(Self::Filter),
            "order" => Some(Self::Order),
            "limit" => Some(Self::Limit),
            "offset" => Some(Self::Offset),
            "group" => Some(Self::Group),
            "mutation-input" => Some(Self::MutationInput),
            _ => None,
        }
    }
}

/// One explicit schema/root mapping registered with the adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphQlRootMapping {
    pub(super) source_operation: GraphQlOperationType,
    pub(super) source_field: String,
    pub(super) operation: QueryOperation,
    pub(super) resource: String,
    pub(super) arguments: BTreeMap<String, GraphQlArgumentRole>,
    pub(super) fields: BTreeMap<String, String>,
    pub(super) aggregates: BTreeMap<String, QueryAggregateFunction>,
}

impl GraphQlRootMapping {
    /// Creates a mapping. Field, argument and aggregate names remain
    /// unsupported until explicitly added.
    #[must_use]
    pub fn new(
        source_operation: GraphQlOperationType,
        source_field: impl Into<String>,
        operation: QueryOperation,
        resource: impl Into<String>,
    ) -> Self {
        Self {
            source_operation,
            source_field: source_field.into(),
            operation,
            resource: resource.into(),
            arguments: BTreeMap::new(),
            fields: BTreeMap::new(),
            aggregates: BTreeMap::new(),
        }
    }

    /// Adds a source argument to semantic-role mapping.
    #[must_use]
    pub fn with_argument(
        mut self,
        source_name: impl Into<String>,
        role: GraphQlArgumentRole,
    ) -> Self {
        self.arguments.insert(source_name.into(), role);
        self
    }

    /// Adds a GraphQL field/enum to canonical domain-field mapping.
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

    /// Adds a GraphQL selection field to canonical aggregate mapping.
    #[must_use]
    pub fn with_aggregate(
        mut self,
        source_name: impl Into<String>,
        aggregate: QueryAggregateFunction,
    ) -> Self {
        self.aggregates.insert(source_name.into(), aggregate);
        self
    }

    /// GraphQL operation family.
    #[must_use]
    pub const fn source_operation(&self) -> GraphQlOperationType {
        self.source_operation
    }

    /// GraphQL root field.
    #[must_use]
    pub fn source_field(&self) -> &str {
        &self.source_field
    }

    /// Canonical operation.
    #[must_use]
    pub const fn operation(&self) -> QueryOperation {
        self.operation
    }

    /// Canonical resource.
    #[must_use]
    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub(super) fn mapped_field(&self, source: &str) -> Result<String, GraphQlAdapterError> {
        self.fields
            .get(source)
            .cloned()
            .ok_or_else(|| GraphQlAdapterError::new(format!("unmapped GraphQL field {source:?}")))
    }

    pub(super) fn mapped_symbol(&self, source: &str) -> Result<String, GraphQlAdapterError> {
        let matches = self
            .fields
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case(source))
            .map(|(_, canonical)| canonical)
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [canonical] => Ok((*canonical).clone()),
            [] => Err(GraphQlAdapterError::new(format!(
                "unmapped GraphQL field symbol {source:?}"
            ))),
            _ => Err(GraphQlAdapterError::new(format!(
                "ambiguous GraphQL field symbol {source:?}"
            ))),
        }
    }
}

/// Explicit registry used by GraphQL lowering. Unknown and duplicate mappings
/// fail closed rather than falling back to schema-name guesses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GraphQlSchemaRegistry {
    roots: BTreeMap<(GraphQlOperationType, String), GraphQlRootMapping>,
}

impl GraphQlSchemaRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            roots: BTreeMap::new(),
        }
    }

    /// Registers one root mapping.
    pub fn register_root(
        &mut self,
        mapping: GraphQlRootMapping,
    ) -> Result<(), GraphQlAdapterError> {
        validate_mapping(&mapping)?;
        let key = (mapping.source_operation, mapping.source_field.clone());
        if self.roots.contains_key(&key) {
            return Err(GraphQlAdapterError::new(format!(
                "duplicate GraphQL {} root mapping {:?}",
                mapping.source_operation.as_str(),
                mapping.source_field
            )));
        }
        self.roots.insert(key, mapping);
        Ok(())
    }

    /// Loads the documented JSON registry shape used by shared parity fixtures.
    pub fn from_json(value: &Value) -> Result<Self, GraphQlAdapterError> {
        let roots = value
            .get("roots")
            .and_then(Value::as_array)
            .ok_or_else(|| GraphQlAdapterError::new("registry.roots must be an array"))?;
        let mut registry = Self::new();
        for root in roots {
            let source_operation = required_string(root, "sourceOperation")?;
            let source_operation =
                GraphQlOperationType::parse(source_operation).ok_or_else(|| {
                    GraphQlAdapterError::new(format!(
                        "unsupported GraphQL operation mapping {source_operation:?}"
                    ))
                })?;
            let operation_label = required_string(root, "operation")?;
            let operation = QueryOperation::parse(operation_label).ok_or_else(|| {
                GraphQlAdapterError::new(format!(
                    "unsupported canonical operation {operation_label:?}"
                ))
            })?;
            let mut mapping = GraphQlRootMapping::new(
                source_operation,
                required_string(root, "sourceField")?,
                operation,
                required_string(root, "resource")?,
            );
            for (name, role) in optional_object(root, "arguments")? {
                let role = role.as_str().ok_or_else(|| {
                    GraphQlAdapterError::new("GraphQL argument roles must be strings")
                })?;
                let role = GraphQlArgumentRole::parse(role).ok_or_else(|| {
                    GraphQlAdapterError::new(format!("unsupported argument role {role:?}"))
                })?;
                mapping = mapping.with_argument(name, role);
            }
            for (name, canonical) in optional_object(root, "fields")? {
                let canonical = canonical.as_str().ok_or_else(|| {
                    GraphQlAdapterError::new("canonical GraphQL fields must be strings")
                })?;
                mapping = mapping.with_field(name, canonical);
            }
            for (name, aggregate) in optional_object(root, "aggregates")? {
                let aggregate = aggregate.as_str().ok_or_else(|| {
                    GraphQlAdapterError::new("GraphQL aggregate mappings must be strings")
                })?;
                let aggregate = QueryAggregateFunction::parse(aggregate).ok_or_else(|| {
                    GraphQlAdapterError::new(format!("unsupported aggregate {aggregate:?}"))
                })?;
                mapping = mapping.with_aggregate(name, aggregate);
            }
            registry.register_root(mapping)?;
        }
        Ok(registry)
    }

    pub(super) fn root(
        &self,
        operation: GraphQlOperationType,
        source_field: &str,
    ) -> Result<&GraphQlRootMapping, GraphQlAdapterError> {
        self.roots
            .get(&(operation, source_field.to_string()))
            .ok_or_else(|| {
                GraphQlAdapterError::new(format!(
                    "unmapped GraphQL {} root field {source_field:?}",
                    operation.as_str()
                ))
            })
    }
}

/// Error returned for invalid registries, syntax, or semantic mappings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphQlAdapterError {
    message: String,
}

impl GraphQlAdapterError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GraphQlAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for GraphQlAdapterError {}

fn validate_mapping(mapping: &GraphQlRootMapping) -> Result<(), GraphQlAdapterError> {
    if mapping.source_field.is_empty() || mapping.resource.is_empty() {
        return Err(GraphQlAdapterError::new(
            "GraphQL root and canonical resource names must not be empty",
        ));
    }
    if mapping.arguments.keys().any(String::is_empty)
        || mapping.fields.keys().any(String::is_empty)
        || mapping.fields.values().any(String::is_empty)
        || mapping.aggregates.keys().any(String::is_empty)
    {
        return Err(GraphQlAdapterError::new(
            "GraphQL mapping names and canonical fields must not be empty",
        ));
    }
    if mapping.source_operation == GraphQlOperationType::Query
        && mapping.operation != QueryOperation::Select
    {
        return Err(GraphQlAdapterError::new(
            "GraphQL query roots must map to the select operation",
        ));
    }
    if mapping.source_operation == GraphQlOperationType::Mutation
        && mapping.operation == QueryOperation::Select
    {
        return Err(GraphQlAdapterError::new(
            "GraphQL mutation roots must map to insert, update, or delete",
        ));
    }
    let unique_roles = mapping.arguments.values().copied().collect::<BTreeSet<_>>();
    if unique_roles.len() != mapping.arguments.len() {
        return Err(GraphQlAdapterError::new(
            "a GraphQL root mapping cannot assign the same semantic role twice",
        ));
    }
    let mut case_insensitive_fields = BTreeSet::new();
    if mapping
        .fields
        .keys()
        .any(|name| !case_insensitive_fields.insert(name.to_ascii_lowercase()))
    {
        return Err(GraphQlAdapterError::new(
            "GraphQL field symbol mappings must not be case-ambiguous",
        ));
    }
    if mapping
        .aggregates
        .keys()
        .any(|name| mapping.fields.contains_key(name))
    {
        return Err(GraphQlAdapterError::new(
            "a GraphQL selection cannot map to both a field and an aggregate",
        ));
    }
    Ok(())
}

fn required_string<'a>(value: &'a Value, key: &str) -> Result<&'a str, GraphQlAdapterError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| GraphQlAdapterError::new(format!("registry root {key:?} must be a string")))
}

fn optional_object<'a>(
    value: &'a Value,
    key: &str,
) -> Result<Vec<(&'a str, &'a Value)>, GraphQlAdapterError> {
    let Some(value) = value.get(key) else {
        return Ok(Vec::new());
    };
    let object = value.as_object().ok_or_else(|| {
        GraphQlAdapterError::new(format!("registry root {key:?} must be an object"))
    })?;
    Ok(object
        .iter()
        .map(|(name, value)| (name.as_str(), value))
        .collect())
}
