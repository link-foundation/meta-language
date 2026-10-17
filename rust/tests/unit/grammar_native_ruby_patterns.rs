//! The exact Ruby pattern CI cases and bounded controls use the shared fixture.
use super::issue_195_native_grammar_rows::{
    Rows, oracle_rows, parser, rebuilt, rebuilt_recovery, source,
};
use meta_language::FeatureParseOptions;
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/ruby.lino");
const PROJECTION: &str = include_str!("../../../parity/fixtures/native-grammars/ruby-profile.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/ruby-pattern-ci.lino");

#[test]
fn ruby_pattern_cases_match_exact_oracle_rows_with_default_limits() {
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    let projection: Value = super::project_data::read_project_data(PROJECTION).unwrap();
    let rows = Rows::new(&projection);
    let parser = parser(GRAMMAR);
    let language = tree_sitter_ruby::LANGUAGE.into();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 6);
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .chain(fixture["controls"].as_array().unwrap())
    {
        let source = source(case);
        let (expected, recovers) = oracle_rows(&language, source);
        assert!(!recovers, "{source}");
        assert_eq!(Value::Array(expected), case["rows"], "{source}");
        let actual = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(actual.ok, "{source}: {:?}", actual.rejection);
        assert!(actual.ambiguities.is_empty(), "{source}");
        let tree = actual.tree.unwrap();
        assert_eq!(
            Value::Array(rows.rows(&tree, source)),
            case["rows"],
            "{source}"
        );
        assert_eq!(rebuilt(&tree), source);
    }
    for case in fixture["rejections"].as_array().unwrap() {
        let source = source(case);
        assert!(oracle_rows(&language, source).1, "{source}");
        let actual = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert!(!actual.ok, "{source}");
        let recovered = parser
            .parse_tree(
                source.as_bytes(),
                &FeatureParseOptions {
                    error_recovery: Some(true),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap();
        assert_eq!(recovered.rejection.unwrap().reason, "recovered");
        assert_eq!(rebuilt_recovery(&recovered.tree.unwrap()), source);
    }
}
