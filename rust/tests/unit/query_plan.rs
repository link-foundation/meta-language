use meta_language::{
    lower_graphql, lower_sql, lower_sql_cst, GraphQlSchemaRegistry, LinkNetwork, LinkType,
    ParseConfiguration, QueryAuthorization, SqlSchemaRegistry,
};
use serde_json::Value;

const SQL_FIXTURES: &str = include_str!("../../../parity/fixtures/sql-query-plans.json");
const GRAPHQL_FIXTURES: &str = include_str!("../../../parity/fixtures/graphql-query-plans.json");

fn fixtures(source: &str) -> Value {
    serde_json::from_str(source).expect("query-plan fixtures are valid JSON")
}

fn sql_registry(fixtures: &Value) -> SqlSchemaRegistry {
    SqlSchemaRegistry::from_json(&fixtures["registry"]).expect("SQL registry is valid")
}

#[test]
fn sql_crud_and_query_concepts_lower_to_shared_canonical_fixtures() {
    let fixtures = fixtures(SQL_FIXTURES);
    let registry = sql_registry(&fixtures);
    for case in fixtures["cases"].as_array().expect("cases are an array") {
        let profile = case["profile"].as_str().expect("profile is a string");
        let sql = case["sql"].as_str().expect("SQL is a string");
        let lowered = lower_sql(sql, profile, &registry)
            .unwrap_or_else(|error| panic!("fixture {:?} failed to lower: {error}", case["name"]));

        assert_eq!(lowered.plan().canonical_value(), case["canonicalPlan"]);
        assert_eq!(lowered.plan().authorization(), QueryAuthorization::Required);
        let evidence = lowered
            .plan()
            .source_evidence()
            .first()
            .expect("statement evidence is retained");
        assert_eq!(evidence.role(), format!("statement:{profile}"));
        assert_eq!(evidence.span().byte_range().end(), sql.len());
    }
}

#[test]
fn vendor_profiles_normalize_the_common_subset_to_one_plan() {
    let fixtures = fixtures(SQL_FIXTURES);
    let registry = sql_registry(&fixtures);
    let sql = fixtures["normalizationSql"]
        .as_str()
        .expect("normalization SQL is a string");
    let expected = lower_sql(sql, "sql-ansi", &registry)
        .expect("ANSI SQL lowers")
        .plan()
        .canonical_json();

    for profile in fixtures["profiles"]
        .as_array()
        .expect("profiles are an array")
    {
        let profile = profile.as_str().expect("profile is a string");
        let lowered = lower_sql(sql, profile, &registry)
            .unwrap_or_else(|error| panic!("{profile} should lower: {error}"));
        assert_eq!(lowered.plan().canonical_json(), expected, "{profile}");
        assert_eq!(
            lowered.plan().source_evidence()[0].role(),
            format!("statement:{profile}")
        );
    }
}

#[test]
fn malformed_unmapped_and_unrepresentable_statements_fail_closed() {
    let fixtures = fixtures(SQL_FIXTURES);
    let registry = sql_registry(&fixtures);
    for case in fixtures["invalid"].as_array().expect("invalid is an array") {
        let profile = case["profile"].as_str().expect("profile is a string");
        let sql = case["sql"].as_str().expect("SQL is a string");
        assert!(
            lower_sql(sql, profile, &registry).is_err(),
            "invalid SQL unexpectedly lowered: {sql}"
        );
    }
    assert!(lower_sql("SELECT id FROM users", "sql-unknown", &registry).is_err());
}

#[test]
fn sql_lowering_retains_full_match_cst_and_semantic_link_evidence() {
    let fixtures = fixtures(SQL_FIXTURES);
    let registry = sql_registry(&fixtures);
    let sql = "SELECT COUNT(*) AS total FROM users WHERE active = TRUE";
    let network = LinkNetwork::parse(sql, "sql-ansi", ParseConfiguration::default());
    let plan = lower_sql_cst(&network, "sql-ansi", &registry).expect("clean CST lowers");
    assert_eq!(
        plan.source_evidence()[0].span().byte_range().end(),
        sql.len()
    );

    let lowered = lower_sql(sql, "sql-ansi", &registry).expect("SQL lowers");
    let root = lowered
        .network()
        .link(lowered.root_link())
        .expect("semantic root exists");
    assert_eq!(root.metadata().link_type(), Some(LinkType::Semantic));
    assert_eq!(root.metadata().term(), Some("executable-query-plan"));
    assert!(root.references().iter().any(|id| {
        lowered
            .network()
            .link(*id)
            .is_some_and(|link| link.metadata().link_type() == Some(LinkType::Syntax))
    }));
}

#[test]
fn equivalent_sql_and_graphql_fixture_produce_the_same_plan() {
    let sql_fixtures = fixtures(SQL_FIXTURES);
    let graphql_fixtures = fixtures(GRAPHQL_FIXTURES);
    assert_eq!(
        sql_fixtures["crossFrontend"]["fixture"],
        "graphql-query-plans.json"
    );
    let case_name = sql_fixtures["crossFrontend"]["case"]
        .as_str()
        .expect("cross-frontend case is a string");
    let graphql_case = graphql_fixtures["cases"]
        .as_array()
        .expect("GraphQL cases are an array")
        .iter()
        .find(|case| case["name"] == case_name)
        .expect("cross-frontend GraphQL case exists");

    let sql = graphql_case["equivalentSql"]
        .as_str()
        .expect("equivalent SQL is present");
    let sql_lowered =
        lower_sql(sql, "sql-ansi", &sql_registry(&sql_fixtures)).expect("equivalent SQL lowers");
    let graphql_registry = GraphQlSchemaRegistry::from_json(&graphql_fixtures["registry"])
        .expect("GraphQL registry is valid");
    let graphql_lowered = lower_graphql(
        graphql_case["source"]
            .as_str()
            .expect("GraphQL source is a string"),
        &graphql_registry,
    )
    .expect("GraphQL fixture lowers");

    assert_eq!(
        sql_lowered.plan().canonical_json(),
        graphql_lowered.plan().canonical_json()
    );
    assert_eq!(
        sql_lowered.plan().canonical_value(),
        graphql_case["canonicalPlan"]
    );
}
