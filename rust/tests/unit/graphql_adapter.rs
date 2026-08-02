use meta_language::{
    lower_graphql, ByteRange, GraphQlOperationType, GraphQlRootMapping, GraphQlSchemaRegistry,
    LinkType, Point, QueryAggregate, QueryAggregateFunction, QueryComparisonOperator, QueryFilter,
    QueryOperation, QueryOrder, QueryPlan, QuerySortDirection, QuerySourceEvidence, QueryValue,
    SourceSpan,
};
use serde_json::Value;

const FIXTURES: &str = include_str!("../../../parity/fixtures/graphql-query-plans.json");

fn registry_and_cases() -> (GraphQlSchemaRegistry, Vec<Value>) {
    let fixture: Value = serde_json::from_str(FIXTURES).expect("fixture is valid JSON");
    let registry =
        GraphQlSchemaRegistry::from_json(&fixture["registry"]).expect("fixture registry is valid");
    let cases = fixture["cases"]
        .as_array()
        .expect("fixture cases are an array")
        .clone();
    (registry, cases)
}

#[test]
fn shared_graphql_fixtures_lower_to_canonical_query_plans() {
    let (registry, cases) = registry_and_cases();

    for fixture in cases {
        let source = fixture["source"].as_str().expect("source is a string");
        let lowered = lower_graphql(source, &registry).expect("fixture lowers");
        let actual: Value =
            serde_json::from_str(&lowered.plan().canonical_json()).expect("canonical plan is JSON");

        assert_eq!(actual, fixture["canonicalPlan"], "{}", fixture["name"]);
        assert!(!lowered.plan().source_evidence().is_empty());
        let root = lowered
            .network()
            .link(lowered.root_link())
            .expect("semantic plan root exists");
        assert_eq!(root.metadata().link_type(), Some(LinkType::Semantic));
        assert!(root.references().iter().any(|reference| {
            lowered
                .network()
                .link(*reference)
                .is_some_and(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        }));
    }
}

#[test]
fn graphql_lowering_fails_closed_for_unmapped_or_ambiguous_input() {
    let (registry, _) = registry_and_cases();

    for source in [
        "query { unknown { id } }",
        "query { users { unknown } }",
        "query { users(unmapped: 1) { id } }",
        "query { users { id } users { name } }",
        "query { users { id }",
        "query { users(first: 01) { id } }",
        "query { users(first: 9007199254740993) { id } }",
        "query { result: users { id } }",
        "query { users { identifier: id } }",
        "query { users { sum } }",
        "mutation { createUser(input: {name: {unmapped: \"Ada\"}}) { id } }",
    ] {
        assert!(
            lower_graphql(source, &registry).is_err(),
            "must reject: {source}"
        );
    }

    let mapping = GraphQlRootMapping::new(
        GraphQlOperationType::Query,
        "users",
        QueryOperation::Select,
        "user",
    );
    let mut duplicates = GraphQlSchemaRegistry::new();
    duplicates
        .register_root(mapping.clone())
        .expect("first mapping is accepted");
    assert!(duplicates.register_root(mapping).is_err());

    let empty_field = GraphQlRootMapping::new(
        GraphQlOperationType::Query,
        "emptyField",
        QueryOperation::Select,
        "user",
    )
    .with_field("id", "");
    assert!(GraphQlSchemaRegistry::new()
        .register_root(empty_field)
        .is_err());

    let ambiguous_field = GraphQlRootMapping::new(
        GraphQlOperationType::Query,
        "ambiguousField",
        QueryOperation::Select,
        "user",
    )
    .with_field("id", "user.id")
    .with_field("ID", "legacy.id");
    assert!(GraphQlSchemaRegistry::new()
        .register_root(ambiguous_field)
        .is_err());
}

#[test]
fn graphql_source_evidence_uses_utf8_byte_offsets() {
    let (registry, _) = registry_and_cases();
    let source = "mutation { createUser(input: {name: \"Zoë\", status: ACTIVE}) { id } }";
    let lowered = lower_graphql(source, &registry).expect("Unicode literal lowers");
    let evidence = lowered
        .plan()
        .source_evidence()
        .iter()
        .find(|entry| entry.role() == "projection:user.id")
        .expect("projection evidence exists");

    assert_eq!(
        evidence.span().byte_range().start(),
        source.rfind("id").expect("projection is present")
    );
}

#[test]
fn public_query_plan_api_supports_additional_frontends() {
    let mut plan = QueryPlan::new(QueryOperation::Select, "user");
    plan.add_projection("id");
    plan.set_filter(QueryFilter::Compare {
        field: "active".into(),
        operator: QueryComparisonOperator::Equal,
        value: QueryValue::Boolean(true),
    });
    plan.add_order(QueryOrder::new("id", QuerySortDirection::Ascending));
    plan.set_pagination(Some(10), Some(2));
    plan.add_group_by("active");
    plan.add_aggregate(QueryAggregate::new(
        QueryAggregateFunction::Count,
        None,
        Some("total".into()),
    ));
    plan.add_source_evidence(QuerySourceEvidence::new(
        "root",
        SourceSpan::new(ByteRange::new(0, 4), Point::new(0, 0), Point::new(0, 4)),
    ));

    assert_eq!(plan.projection(), &["id"]);
    assert_eq!(plan.order()[0].field(), "id");
    assert_eq!(plan.aggregates()[0].alias(), Some("total"));
    assert_eq!(plan.source_evidence()[0].role(), "root");

    let mut update = QueryPlan::new(QueryOperation::Update, "user");
    update.set_mutation_value("name", QueryValue::String("Ada".into()));
    assert_eq!(update.mutation()["name"], QueryValue::String("Ada".into()));
}
