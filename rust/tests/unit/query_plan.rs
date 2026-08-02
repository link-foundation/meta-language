use std::sync::Arc;

use meta_language::{
    lower_sql, lower_sql_cst, LinkNetwork, LinkType, ParseConfiguration, QueryAuthorization,
    QueryFrontend, QueryPlan, QueryPlanError, QueryPlanRegistry, SourceEvidence,
};
use serde_json::Value;

const FIXTURES: &str = include_str!("../../../parity/query-plan-fixtures.json");

fn fixtures() -> Value {
    serde_json::from_str(FIXTURES).expect("query-plan fixtures are valid JSON")
}

#[test]
fn sql_crud_and_query_concepts_lower_to_shared_canonical_fixtures() {
    let fixtures = fixtures();
    for case in fixtures["cases"].as_array().expect("cases are an array") {
        let profile = case["profile"].as_str().expect("profile is a string");
        let sql = case["sql"].as_str().expect("sql is a string");
        let plan = lower_sql(sql, profile)
            .unwrap_or_else(|error| panic!("fixture {:?} failed to lower: {error}", case["name"]));

        assert_eq!(plan.canonical_json(), case["plan"]);
        assert_eq!(plan.authorization(), QueryAuthorization::Required);
        assert_eq!(plan.evidence().language(), profile);
        assert!(plan.evidence().syntax_link().is_some());
        assert_eq!(
            plan.evidence().span().byte_range().end(),
            sql.len(),
            "fixture {:?} should retain the complete source span",
            case["name"]
        );
    }
}

#[test]
fn vendor_profiles_normalize_the_common_subset_to_one_operation() {
    let fixtures = fixtures();
    let sql = fixtures["normalizationSql"]
        .as_str()
        .expect("normalization SQL is a string");
    let profiles = fixtures["profiles"]
        .as_array()
        .expect("profiles are an array");

    let expected = lower_sql(sql, "sql-ansi")
        .expect("ANSI fixture lowers")
        .canonical_json();
    for profile in profiles {
        let profile = profile.as_str().expect("profile is a string");
        let plan = lower_sql(sql, profile)
            .unwrap_or_else(|error| panic!("{profile} should lower: {error}"));
        assert_eq!(plan.canonical_json(), expected, "profile {profile}");
        assert_eq!(plan.evidence().language(), profile);
    }
}

#[test]
fn malformed_and_semantically_invalid_statements_fail_closed() {
    let fixtures = fixtures();
    for case in fixtures["invalid"].as_array().expect("invalid is an array") {
        let profile = case["profile"].as_str().expect("profile is a string");
        let sql = case["sql"].as_str().expect("sql is a string");
        assert!(
            lower_sql(sql, profile).is_err(),
            "invalid SQL unexpectedly lowered: {sql}"
        );
    }

    assert!(lower_sql("SELECT id FROM users", "sql-unknown").is_err());
}

#[test]
fn sql_lowering_uses_full_match_cst_evidence_and_can_declare_semantic_links() {
    let sql = "SELECT COUNT(*) AS total FROM users WHERE active = TRUE";
    let mut network = LinkNetwork::parse(sql, "sql-ansi", ParseConfiguration::default());
    let plan = lower_sql_cst(&network, "sql-ansi").expect("clean CST lowers");
    let syntax_link = plan.evidence().syntax_link().expect("CST link is retained");

    let declared = plan.declare_in(&mut network);
    let root = network.link(declared.root()).expect("plan root exists");
    assert_eq!(root.metadata().link_type(), Some(LinkType::Semantic));
    assert_eq!(root.metadata().term(), Some("query-plan"));
    assert!(declared
        .links()
        .iter()
        .filter_map(|id| network.link(*id))
        .any(|link| link.references().contains(&syntax_link)));
    assert!(declared
        .links()
        .iter()
        .filter_map(|id| network.link(*id))
        .any(|link| link.metadata().term() == Some("query-aggregate:count")));
}

#[derive(Debug)]
struct LinksQueryFrontend {
    operation: meta_language::QueryOperation,
}

impl QueryFrontend for LinksQueryFrontend {
    fn lower(&self, _source: &str, language: &str) -> Result<QueryPlan, QueryPlanError> {
        Ok(QueryPlan::new(
            self.operation.clone(),
            SourceEvidence::synthetic(language),
        ))
    }
}

#[test]
fn a_second_frontend_can_reuse_the_same_canonical_plan() {
    let fixtures = fixtures();
    let frontend = &fixtures["nonSqlFrontend"];
    let language = frontend["language"]
        .as_str()
        .expect("frontend language is a string");
    let source = frontend["source"]
        .as_str()
        .expect("frontend source is a string");
    let equivalent_sql = frontend["equivalentSql"]
        .as_str()
        .expect("equivalent SQL is a string");
    let sql_plan = lower_sql(equivalent_sql, "sql-ansi").expect("SQL lowers");
    let mut registry = QueryPlanRegistry::new();
    registry.register(
        language,
        Arc::new(LinksQueryFrontend {
            operation: sql_plan.operation().clone(),
        }),
    );

    let links_plan = registry
        .lower(source, language)
        .expect("custom frontend lowers");
    assert_eq!(links_plan.operation(), sql_plan.operation());
    assert_eq!(links_plan.evidence().language(), language);
}
