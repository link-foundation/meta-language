//! The same generated scanner data and concrete trees as the JavaScript suite.
use super::issue_195_observations::{Observation, record};
use meta_language::{FeatureParseOptions, compile_feature_grammar, parse_grammar_links};
use serde_json::Value;

fn check_cases(text: &str) {
    let fixtures: Value = serde_json::from_str(text).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        let grammar = parse_grammar_links(fixture["listing"].as_str().unwrap()).unwrap();
        let parser =
            compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).unwrap();
        for input in fixture["accept"].as_array().unwrap() {
            let input = input.as_str().unwrap();
            let outcome = parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .unwrap();
            assert!(
                outcome.ok,
                "{}: {input:?}: {:?}",
                fixture["name"], outcome.rejection
            );
            let tree = outcome.tree.as_ref().unwrap();
            assert_eq!(
                tree.render(),
                fixture["trees"][input].as_str().unwrap(),
                "{}: {input:?}",
                fixture["name"]
            );
        }
        for input in fixture["reject"].as_array().unwrap() {
            let input = input.as_str().unwrap();
            assert!(
                !parser
                    .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                    .unwrap()
                    .ok,
                "{}: {input:?}",
                fixture["name"]
            );
        }
    }
}

#[test]
fn generated_scanner_families_preserve_text_and_reject_truncated_delimiters() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-families.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-DELIMITER-FAMILIES",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-delimiter-families",
        fixture_file: "parity/fixtures/scanner-families.json",
        assertions: &[
            "nestedComments",
            "escapedStrings",
            "quotedContent",
            "interpolationBoundaries",
            "alternateNestedDelimiters",
        ],
        test_name: "generated_scanner_families_preserve_text_and_reject_truncated_delimiters",
    });
}

#[test]
fn counted_scanner_delimiters_preserve_content_and_clear_state_between_tokens() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-counted-delimiters.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-COUNTED-DELIMITERS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-counted-delimiters",
        fixture_file: "parity/fixtures/scanner-counted-delimiters.json",
        assertions: &["rustStyleRawDelimiters", "luaStyleLongDelimiters"],
        test_name: "counted_scanner_delimiters_preserve_content_and_clear_state_between_tokens",
    });
}

#[test]
fn generated_delimiter_runs_and_line_boundaries_match_shared_trees() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-delimiter-runs.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-DELIMITER-RUNS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-delimiter-runs",
        fixture_file: "parity/fixtures/scanner-delimiter-runs.json",
        assertions: &["delimiterRuns", "lineBoundaries"],
        test_name: "generated_delimiter_runs_and_line_boundaries_match_shared_trees",
    });
}
