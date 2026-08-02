//! Registry-driven lowering from validated GraphQL operations to query plans.

mod parser;
mod registry;

use std::collections::BTreeMap;

use crate::configuration::ParseConfiguration;
use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::query_plan::{
    QueryAggregate, QueryAggregateFunction, QueryComparisonOperator, QueryFilter, QueryOperation,
    QueryOrder, QueryPlan, QuerySortDirection, QuerySourceEvidence, QueryValue,
};
use crate::source::{ByteRange, Point, SourceSpan};

use parser::{ByteSpan, Field, ValueKind, ValueNode};
pub use registry::{
    GraphQlAdapterError, GraphQlArgumentRole, GraphQlOperationType, GraphQlRootMapping,
    GraphQlSchemaRegistry,
};

/// Query plan and its provenance-connected links network.
#[derive(Clone, Debug, PartialEq)]
pub struct LoweredQueryPlan {
    plan: QueryPlan,
    network: LinkNetwork,
    root_link: LinkId,
}

impl LoweredQueryPlan {
    /// Canonical executable plan.
    #[must_use]
    pub const fn plan(&self) -> &QueryPlan {
        &self.plan
    }

    /// Original GraphQL CST plus attached semantic plan links.
    #[must_use]
    pub const fn network(&self) -> &LinkNetwork {
        &self.network
    }

    /// Root semantic link for the canonical plan.
    #[must_use]
    pub const fn root_link(&self) -> LinkId {
        self.root_link
    }

    /// Splits the result into its public components.
    #[must_use]
    pub fn into_parts(self) -> (QueryPlan, LinkNetwork, LinkId) {
        (self.plan, self.network, self.root_link)
    }
}

/// Validates and lowers exactly one GraphQL operation/root field.
pub fn lower_graphql(
    source: &str,
    registry: &GraphQlSchemaRegistry,
) -> Result<LoweredQueryPlan, GraphQlAdapterError> {
    let mut network = LinkNetwork::parse(source, "GraphQL", ParseConfiguration::default());
    if !network.verify_full_match(None).is_clean() {
        return Err(GraphQlAdapterError::new(
            "GraphQL CST validation failed; semantic lowering was not attempted",
        ));
    }
    let document = parser::parse(source)?;
    let [root] = document.root_fields.as_slice() else {
        return Err(GraphQlAdapterError::new(
            "exactly one GraphQL root field is required",
        ));
    };
    if root.alias.is_some() {
        return Err(GraphQlAdapterError::new(
            "GraphQL root aliases are not represented by the query plan",
        ));
    }
    let mapping = registry.root(document.operation, &root.name)?;
    let mut plan = QueryPlan::new(mapping.operation, &mapping.resource);
    plan.source_evidence
        .push(evidence(source, "root", root.span));
    lower_arguments(source, root, mapping, &mut plan)?;
    lower_selection(source, root, mapping, &mut plan)?;
    validate_plan(&plan)?;
    let root_link = attach_plan_links(&mut network, &plan);
    Ok(LoweredQueryPlan {
        plan,
        network,
        root_link,
    })
}

fn lower_arguments(
    source: &str,
    root: &Field,
    mapping: &GraphQlRootMapping,
    plan: &mut QueryPlan,
) -> Result<(), GraphQlAdapterError> {
    for argument in &root.arguments {
        let role = mapping.arguments.get(&argument.name).ok_or_else(|| {
            GraphQlAdapterError::new(format!("unmapped GraphQL argument {:?}", argument.name))
        })?;
        match role {
            GraphQlArgumentRole::Filter => {
                plan.filter = Some(lower_filter(&argument.value, mapping)?);
            }
            GraphQlArgumentRole::Order => {
                plan.order = lower_order(&argument.value, mapping)?;
            }
            GraphQlArgumentRole::Limit => {
                plan.limit = Some(non_negative_integer(&argument.value, "limit")?);
            }
            GraphQlArgumentRole::Offset => {
                plan.offset = Some(non_negative_integer(&argument.value, "offset")?);
            }
            GraphQlArgumentRole::Group => {
                plan.group_by = lower_group(&argument.value, mapping)?;
            }
            GraphQlArgumentRole::MutationInput => {
                plan.mutation = lower_mutation(&argument.value, mapping)?;
            }
        }
        plan.source_evidence.push(evidence(
            source,
            format!("argument:{}", role.as_str()),
            argument.value.span,
        ));
    }
    Ok(())
}

fn lower_selection(
    source: &str,
    root: &Field,
    mapping: &GraphQlRootMapping,
    plan: &mut QueryPlan,
) -> Result<(), GraphQlAdapterError> {
    if root.selection.is_empty() {
        return Err(GraphQlAdapterError::new(
            "a mapped GraphQL root must have a projection selection",
        ));
    }
    for field in &root.selection {
        if !field.selection.is_empty() {
            return Err(GraphQlAdapterError::new(format!(
                "nested projection field {:?} has no explicit mapping",
                field.name
            )));
        }
        if let Some(function) = mapping.aggregates.get(&field.name) {
            let aggregate_field = aggregate_field(field, mapping)?;
            if *function != QueryAggregateFunction::Count && aggregate_field.is_none() {
                return Err(GraphQlAdapterError::new(format!(
                    "aggregate {:?} requires a mapped field argument",
                    field.name
                )));
            }
            plan.aggregates.push(QueryAggregate {
                function: *function,
                field: aggregate_field,
                alias: field.alias.clone(),
            });
            plan.source_evidence.push(evidence(
                source,
                format!("aggregate:{}", function.as_str()),
                field.span,
            ));
            continue;
        }
        if !field.arguments.is_empty() {
            return Err(GraphQlAdapterError::new(format!(
                "non-aggregate projection {:?} cannot have arguments",
                field.name
            )));
        }
        if field.alias.is_some() {
            return Err(GraphQlAdapterError::new(format!(
                "projection alias on {:?} is not represented by the query plan",
                field.name
            )));
        }
        let canonical = mapping.mapped_field(&field.name)?;
        if !plan.projection.contains(&canonical) {
            plan.projection.push(canonical.clone());
        }
        plan.source_evidence.push(evidence(
            source,
            format!("projection:{canonical}"),
            field.span,
        ));
    }
    Ok(())
}

fn aggregate_field(
    field: &Field,
    mapping: &GraphQlRootMapping,
) -> Result<Option<String>, GraphQlAdapterError> {
    match field.arguments.as_slice() {
        [] => Ok(None),
        [argument] if argument.name == "field" => {
            Ok(Some(mapping.mapped_symbol(symbol(&argument.value)?)?))
        }
        _ => Err(GraphQlAdapterError::new(format!(
            "aggregate {:?} accepts only an optional field argument",
            field.name
        ))),
    }
}

fn lower_filter(
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<QueryFilter, GraphQlAdapterError> {
    let ValueKind::Object(fields) = &value.value else {
        return Err(GraphQlAdapterError::new("filter arguments must be objects"));
    };
    let mut clauses = Vec::new();
    for (name, value) in fields {
        match name.as_str() {
            "and" => clauses.push(QueryFilter::And(filter_list(value, mapping)?)),
            "or" => clauses.push(QueryFilter::Or(filter_list(value, mapping)?)),
            "not" => clauses.push(QueryFilter::Not(Box::new(lower_filter(value, mapping)?))),
            _ => clauses.extend(field_filter(name, value, mapping)?),
        }
    }
    collapse_and(clauses, "empty filter objects are unsupported")
}

fn filter_list(
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<Vec<QueryFilter>, GraphQlAdapterError> {
    let ValueKind::List(values) = &value.value else {
        return Err(GraphQlAdapterError::new(
            "GraphQL and/or filter values must be lists",
        ));
    };
    if values.is_empty() {
        return Err(GraphQlAdapterError::new(
            "GraphQL and/or filter lists must not be empty",
        ));
    }
    values
        .iter()
        .map(|value| lower_filter(value, mapping))
        .collect()
}

fn field_filter(
    source_field: &str,
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<Vec<QueryFilter>, GraphQlAdapterError> {
    let field = mapping.mapped_field(source_field)?;
    let ValueKind::Object(operators) = &value.value else {
        return Ok(vec![QueryFilter::Compare {
            field,
            operator: QueryComparisonOperator::Equal,
            value: query_value(value)?,
        }]);
    };
    if operators.is_empty() {
        return Err(GraphQlAdapterError::new(
            "empty comparison objects are unsupported",
        ));
    }
    operators
        .iter()
        .map(|(name, value)| {
            let operator = QueryComparisonOperator::from_graphql_key(name).ok_or_else(|| {
                GraphQlAdapterError::new(format!("unsupported filter operator {name:?}"))
            })?;
            if operator == QueryComparisonOperator::IsNull
                && !matches!(value.value, ValueKind::Boolean(_))
            {
                return Err(GraphQlAdapterError::new("isNull requires a boolean value"));
            }
            if matches!(
                operator,
                QueryComparisonOperator::In | QueryComparisonOperator::NotIn
            ) && !matches!(value.value, ValueKind::List(_))
            {
                return Err(GraphQlAdapterError::new("in/notIn requires a list value"));
            }
            Ok(QueryFilter::Compare {
                field: field.clone(),
                operator,
                value: query_value(value)?,
            })
        })
        .collect()
}

fn collapse_and(
    mut clauses: Vec<QueryFilter>,
    empty_message: &str,
) -> Result<QueryFilter, GraphQlAdapterError> {
    match clauses.len() {
        0 => Err(GraphQlAdapterError::new(empty_message)),
        1 => Ok(clauses.remove(0)),
        _ => Ok(QueryFilter::And(clauses)),
    }
}

fn lower_order(
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<Vec<QueryOrder>, GraphQlAdapterError> {
    let entries = match &value.value {
        ValueKind::List(entries) => entries.as_slice(),
        ValueKind::Object(_) => std::slice::from_ref(value),
        _ => {
            return Err(GraphQlAdapterError::new(
                "order arguments must be objects or lists of objects",
            ));
        }
    };
    if entries.is_empty() {
        return Err(GraphQlAdapterError::new("order lists must not be empty"));
    }
    entries
        .iter()
        .map(|entry| lower_order_entry(entry, mapping))
        .collect()
}

fn lower_order_entry(
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<QueryOrder, GraphQlAdapterError> {
    let ValueKind::Object(fields) = &value.value else {
        return Err(GraphQlAdapterError::new(
            "each order entry must be an object",
        ));
    };
    let field_value = object_field(fields, "field")?;
    let direction_value = object_field(fields, "direction")?;
    if fields.len() != 2 {
        return Err(GraphQlAdapterError::new(
            "order entries support only field and direction",
        ));
    }
    Ok(QueryOrder {
        field: mapping.mapped_symbol(symbol(field_value)?)?,
        direction: direction(direction_value)?,
    })
}

fn lower_group(
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<Vec<String>, GraphQlAdapterError> {
    let values = match &value.value {
        ValueKind::List(values) => values.as_slice(),
        ValueKind::Enum(_) | ValueKind::String(_) => std::slice::from_ref(value),
        _ => return Err(GraphQlAdapterError::new("group arguments must name fields")),
    };
    if values.is_empty() {
        return Err(GraphQlAdapterError::new("group lists must not be empty"));
    }
    values
        .iter()
        .map(|value| mapping.mapped_symbol(symbol(value)?))
        .collect()
}

fn lower_mutation(
    value: &ValueNode,
    mapping: &GraphQlRootMapping,
) -> Result<BTreeMap<String, QueryValue>, GraphQlAdapterError> {
    let ValueKind::Object(fields) = &value.value else {
        return Err(GraphQlAdapterError::new(
            "mutation input arguments must be objects",
        ));
    };
    if fields.is_empty() {
        return Err(GraphQlAdapterError::new(
            "mutation inputs must not be empty",
        ));
    }
    fields
        .iter()
        .map(|(field, value)| Ok((mapping.mapped_field(field)?, query_value(value)?)))
        .collect()
}

fn query_value(value: &ValueNode) -> Result<QueryValue, GraphQlAdapterError> {
    match &value.value {
        ValueKind::Null => Ok(QueryValue::Null),
        ValueKind::Boolean(value) => Ok(QueryValue::Boolean(*value)),
        ValueKind::Integer(value) => safe_integer(*value).map(QueryValue::Integer),
        ValueKind::Float(value) => Ok(QueryValue::Float(*value)),
        ValueKind::String(value) | ValueKind::Enum(value) => Ok(QueryValue::String(value.clone())),
        ValueKind::List(values) => values
            .iter()
            .map(query_value)
            .collect::<Result<Vec<_>, _>>()
            .map(QueryValue::List),
        ValueKind::Object(_) => Err(GraphQlAdapterError::new(
            "nested GraphQL input objects require explicit field mappings",
        )),
    }
}

fn non_negative_integer(value: &ValueNode, role: &str) -> Result<u64, GraphQlAdapterError> {
    let ValueKind::Integer(value) = value.value else {
        return Err(GraphQlAdapterError::new(format!(
            "GraphQL {role} must be a non-negative integer"
        )));
    };
    safe_integer(value).and_then(|value| {
        u64::try_from(value).map_err(|_| {
            GraphQlAdapterError::new(format!("GraphQL {role} must be a non-negative integer"))
        })
    })
}

fn safe_integer(value: i64) -> Result<i64, GraphQlAdapterError> {
    const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
    if (-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&value) {
        Ok(value)
    } else {
        Err(GraphQlAdapterError::new(
            "GraphQL integer is outside the cross-language safe range",
        ))
    }
}

fn symbol(value: &ValueNode) -> Result<&str, GraphQlAdapterError> {
    match &value.value {
        ValueKind::Enum(value) | ValueKind::String(value) => Ok(value),
        _ => Err(GraphQlAdapterError::new(
            "canonical field references must be GraphQL enums or strings",
        )),
    }
}

fn direction(value: &ValueNode) -> Result<QuerySortDirection, GraphQlAdapterError> {
    match symbol(value)?.to_ascii_lowercase().as_str() {
        "asc" => Ok(QuerySortDirection::Ascending),
        "desc" => Ok(QuerySortDirection::Descending),
        other => Err(GraphQlAdapterError::new(format!(
            "unsupported sort direction {other:?}"
        ))),
    }
}

fn object_field<'a>(
    fields: &'a [(String, ValueNode)],
    name: &str,
) -> Result<&'a ValueNode, GraphQlAdapterError> {
    fields
        .iter()
        .find_map(|(field, value)| (field == name).then_some(value))
        .ok_or_else(|| GraphQlAdapterError::new(format!("missing object field {name:?}")))
}

fn validate_plan(plan: &QueryPlan) -> Result<(), GraphQlAdapterError> {
    if plan.projection.is_empty() && plan.aggregates.is_empty() {
        return Err(GraphQlAdapterError::new(
            "a query plan requires a projection or aggregate",
        ));
    }
    match plan.operation {
        QueryOperation::Insert | QueryOperation::Update if plan.mutation.is_empty() => Err(
            GraphQlAdapterError::new("insert/update mutations require a mapped mutation input"),
        ),
        QueryOperation::Select | QueryOperation::Delete if !plan.mutation.is_empty() => {
            Err(GraphQlAdapterError::new(
                "select/delete operations cannot contain mutation assignments",
            ))
        }
        _ => Ok(()),
    }
}

fn attach_plan_links(network: &mut LinkNetwork, plan: &QueryPlan) -> LinkId {
    let cst_links = plan
        .source_evidence
        .iter()
        .map(|evidence| closest_cst(network, evidence.span()))
        .collect::<Vec<_>>();
    let plan_concept = network.insert_point("executable-query-plan");
    let mut references = vec![plan_concept];
    for (evidence, cst) in plan.source_evidence.iter().zip(cst_links) {
        let concept = network.insert_point(evidence.role());
        let mut child_references = vec![concept];
        if let Some(cst) = cst {
            child_references.push(cst);
        }
        let child = network.insert_dynamic_link(
            &child_references,
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_term(evidence.role())
                .with_language("GraphQL")
                .with_span(evidence.span()),
        );
        references.push(child);
    }
    if let Some(cst) = plan
        .source_evidence
        .first()
        .and_then(|evidence| closest_cst(network, evidence.span()))
    {
        references.push(cst);
    }
    let root_span = plan
        .source_evidence
        .first()
        .map(QuerySourceEvidence::span)
        .expect("lowered plans always retain root evidence");
    network.insert_dynamic_link(
        &references,
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("executable-query-plan")
            .with_language("GraphQL")
            .with_span(root_span),
    )
}

fn closest_cst(network: &LinkNetwork, span: SourceSpan) -> Option<LinkId> {
    let target = span.byte_range();
    network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        .filter_map(|link| Some((link, link.metadata().span()?.byte_range())))
        .filter(|(_, candidate)| {
            candidate.start() <= target.start() && candidate.end() >= target.end()
        })
        .min_by_key(|(_, candidate)| candidate.end() - candidate.start())
        .map(|(link, _)| Link::id(link))
}

fn evidence(source: &str, role: impl Into<String>, span: ByteSpan) -> QuerySourceEvidence {
    QuerySourceEvidence::new(role, source_span(source, span))
}

fn source_span(source: &str, span: ByteSpan) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(span.start, span.end),
        point_at(source, span.start),
        point_at(source, span.end),
    )
}

fn point_at(source: &str, offset: usize) -> Point {
    let prefix = &source[..offset];
    let row = prefix.bytes().filter(|byte| *byte == b'\n').count();
    let column = prefix
        .rsplit_once('\n')
        .map_or(prefix.len(), |(_, line)| line.len());
    Point::new(row, column)
}
