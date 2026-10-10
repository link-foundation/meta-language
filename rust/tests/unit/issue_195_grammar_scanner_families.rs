//! The same generated scanner data and concrete trees as the JavaScript suite.
use super::issue_195_observations::{Observation, record};
use meta_language::{FeatureParseOptions, compile_feature_grammar, parse_grammar_links};
use serde_json::Value;

fn observe_scanner_state(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-STATE-SEMANTICS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-state-semantics",
        fixture_file: "parity/fixtures/scanner-state-semantics.json",
        assertions,
        test_name,
    });
}

#[test]
fn scanner_values_normalize_single_uppercase_scalars_without_expanding_names() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/scanner-state-semantics.json"
    ))
    .unwrap();
    let listing = fixture[0]["listing"].as_str().unwrap();
    let parser = super::issue_195_native_grammar_rows::parser(listing);
    for input in ["Tree:TREE", "ÉléMent:ÉLÉMENT", "ſ:S", "ß:ß"] {
        let tree = super::issue_195_native_grammar_rows::parse(&parser, input).unwrap();
        assert_eq!(super::issue_195_native_grammar_rows::rebuilt(&tree), input);
    }
    for input in ["Tree:TREAT", "ß:SS"] {
        assert!(
            parser
                .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                .unwrap()
                .tree
                .is_none(),
            "{input}"
        );
    }
    observe_scanner_state(
        &["scannerCaseConversionUsesSingleScalars"],
        "scanner_values_normalize_single_uppercase_scalars_without_expanding_names",
    );
}

#[test]
fn scanner_lookahead_predicates_observe_preceding_state_mutations() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/scanner-state-semantics.json"
    ))
    .unwrap();
    let listing = fixture[1]["listing"].as_str().unwrap();
    let parser = super::issue_195_native_grammar_rows::parser(listing);
    for input in ["x", "y", "x"] {
        let outcome = parser
            .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
            .unwrap();
        assert_eq!(outcome.ok, input == "x");
        if let Some(tree) = outcome.tree {
            assert_eq!(super::issue_195_native_grammar_rows::rebuilt(&tree), input);
        }
    }
    observe_scanner_state(
        &[
            "scannerPredicatesSeeCurrentState",
            "scannerStateFailuresRemainIsolated",
        ],
        "scanner_lookahead_predicates_observe_preceding_state_mutations",
    );
}

#[test]
fn external_extras_keep_skipped_prefixes_outside_their_named_token_span() {
    let listing = "(grammar (start source))\n(extra (ref annotation))\n(scanner annotations (tokens annotation) (operations (while (next (class plain (char %20) (char %0A))) (do (skip (class plain (char %20) (char %0A))))) (consume (literal %23)) (while (all (not atEnd) (not (next (literal %0A)))) (do advance)) (emit annotation)))\n(rule source normal (repeat0 (literal x)))";
    let input = "x \n#note";
    let grammar = parse_grammar_links(listing).unwrap();
    let parser = compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).unwrap();
    let outcome = parser
        .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert!(outcome.ok);
    let tree = outcome.tree.as_ref().unwrap();
    let leaves = super::issue_195_native_grammar_rows::leaves(tree);
    let annotation = leaves.iter().find(|child| matches!(child, meta_language::SyntaxTree::Token { kind: Some(kind), .. } if kind == "annotation")).unwrap();
    let meta_language::SyntaxTree::Token {
        start, end, text, ..
    } = annotation
    else {
        panic!("annotation is a token")
    };
    assert_eq!(*start, 3);
    assert_eq!(*end, 8);
    assert_eq!(super::issue_195_native_grammar_rows::text(text), "#note");
    assert_eq!(super::issue_195_native_grammar_rows::rebuilt(tree), input);
}

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
fn wrapped_scanner_tokens_keep_trivia_and_consumed_text_after_a_mark() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-token-spans.json"
    ));
}

#[test]
fn remembered_delimiter_text_matches_shared_trees_and_clears_state() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-remembered-delimiters.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-REMEMBERED-DELIMITERS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-remembered-delimiters",
        fixture_file: "parity/fixtures/scanner-remembered-delimiters.json",
        assertions: &["rememberedDelimiterText"],
        test_name: "remembered_delimiter_text_matches_shared_trees_and_clears_state",
    });
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

#[test]
fn generated_context_tokens_and_counted_line_delimiters_match_shared_trees() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-context-tokens.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-CONTEXT-TOKENS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-context-tokens",
        fixture_file: "parity/fixtures/scanner-context-tokens.json",
        assertions: &[
            "lookaheadBoundaries",
            "contextTokens",
            "lineCountedDelimiters",
        ],
        test_name: "generated_context_tokens_and_counted_line_delimiters_match_shared_trees",
    });
}

#[test]
fn remembered_content_matches_shared_trees_and_clears_state() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-remembered-content.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-REMEMBERED-CONTENT",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-remembered-content",
        fixture_file: "parity/fixtures/scanner-remembered-content.json",
        assertions: &["rememberedLiteralText", "rememberedContentText"],
        test_name: "remembered_content_matches_shared_trees_and_clears_state",
    });
}

#[test]
fn fragment_contexts_and_counted_content_match_shared_trees() {
    check_cases(include_str!(
        "../../../parity/fixtures/scanner-fragments.json"
    ));
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SCANNER-FRAGMENTS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-scanner-fragments",
        fixture_file: "parity/fixtures/scanner-fragments.json",
        assertions: &[
            "pairedFragmentPrefixes",
            "requiredFragmentMarker",
            "delimiterOpeningLookahead",
            "lexicalTokenContext",
            "countedContentPolicies",
        ],
        test_name: "fragment_contexts_and_counted_content_match_shared_trees",
    });
}
