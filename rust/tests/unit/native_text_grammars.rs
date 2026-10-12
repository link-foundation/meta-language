//! The text adapters execute the same generated Links rules as JavaScript.
use super::issue_195_observations::{Observation, record};
use meta_language::{
    FeatureParseOptions, LinkNetwork, LinkQuery, LinkType, ParseConfiguration, SyntaxTree,
    compile_feature_grammar, parse_grammar_links,
};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native-text.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-text-grammars.json");

#[test]
fn native_text_grammar_files_are_identical_in_both_packages() {
    assert_eq!(
        GRAMMAR,
        include_str!("../../../js/src/data/native-text-grammar.lino")
    );
    assert_eq!(
        GRAMMAR,
        include_str!("../../src/data/native-text-grammar.lino")
    );
    assert_eq!(
        include_str!("../../../js/src/data/native-text-metadata.lino"),
        include_str!("../../src/data/native-text-metadata.lino")
    );
}

#[test]
fn native_natural_language_rules_execute_with_lossless_spans_and_diagnostics() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let grammar = parse_grammar_links(GRAMMAR).unwrap();
    let parser = compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).unwrap();
    for case in fixture["natural"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let language = case["language"].as_str().unwrap();
        let parsed = parser
            .parse_tree(
                source.as_bytes(),
                &FeatureParseOptions {
                    start_rule: Some("natural_language_document".to_owned()),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap();
        assert!(parsed.ok, "{language}");
        let Some(SyntaxTree::Node { kind, children, .. }) = parsed.tree else {
            panic!("native document tree required");
        };
        assert_eq!(kind, "natural_language_document");
        let regions: Vec<_> = children
            .iter()
            .map(|region| &source[region.start()..region.end()])
            .collect();
        let expected: Vec<_> = case["regions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        assert_eq!(regions, expected, "{language}");
        assert_eq!(regions.concat(), source);
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        let roots = network.query_links(
            &LinkQuery::by_type(LinkType::Syntax).with_term("natural_language_document"),
        );
        assert_eq!(roots.len(), 1);
        let sentences =
            network.query_links(&LinkQuery::by_type(LinkType::Syntax).with_term("sentence"));
        let represented: Vec<_> = sentences
            .iter()
            .map(|sentence| {
                let range = sentence.metadata().span().unwrap().byte_range();
                &source[range.start()..range.end()]
            })
            .collect();
        assert_eq!(represented, expected, "{language}");
        if case["hasError"].as_bool().unwrap_or(false) {
            assert!(
                !network
                    .query_links(&LinkQuery::by_type(LinkType::Syntax).with_term("ERROR"))
                    .is_empty()
            );
            assert!(!network.verify_full_match(None).is_clean());
        }
    }
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATURAL-LANGUAGE-REPRESENTATION",
        suffix: "behavior",
        fixture_id: "native-text-grammars",
        fixture_file: "parity/fixtures/native-text-grammars.json",
        assertions: &[
            "nativeNaturalLanguageGrammarExecuted",
            "naturalLanguageUnicodeSpansLossless",
            "naturalLanguageSentenceLexemesPreserved",
            "naturalLanguageControlDiagnostics",
        ],
        test_name: "native_natural_language_rules_execute_with_lossless_spans_and_diagnostics",
    });
}

#[test]
fn native_plain_text_retains_every_source_byte_and_control_errors() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    for case in fixture["plain"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let network = LinkNetwork::parse(source, "txt", ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        let lines = network.query_links(&LinkQuery::by_type(LinkType::Syntax).with_term("line"));
        let represented: Vec<_> = lines
            .iter()
            .map(|line| {
                let range = line.metadata().span().unwrap().byte_range();
                &source[range.start()..range.end()]
            })
            .collect();
        let expected: Vec<_> = case["regions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        assert_eq!(represented, expected);
        assert_eq!(
            !network.verify_full_match(None).is_clean(),
            case["hasError"].as_bool().unwrap()
        );
    }
}
