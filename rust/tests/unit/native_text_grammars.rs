//! The text adapters execute the same generated Links rules as JavaScript.
use super::issue_195_observations::{Observation, record};
use meta_language::{
    FeatureParseOptions, LinkNetwork, LinkQuery, LinkType, ParseConfiguration, SyntaxTree,
    compile_feature_grammar, parse_grammar_links,
};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native-text.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-text-grammars.lino");
const SENTENCE_EXAMPLES: &str =
    include_str!("../../../parity/fixtures/natural-language-sentence-abbreviations.lino");

#[test]
fn sentence_abbreviations_preserve_context_and_true_boundaries() {
    let examples: Value = super::project_data::read_project_data(SENTENCE_EXAMPLES).unwrap();
    for example in examples.as_array().unwrap() {
        let language = example["language"].as_str().unwrap();
        let source = example["source"].as_str().unwrap();
        let expected: Vec<_> = example["sentences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|sentence| sentence.as_str().unwrap())
            .collect();
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        let represented: Vec<_> = network
            .query_links(&LinkQuery::by_type(LinkType::Syntax).with_term("sentence"))
            .iter()
            .map(|sentence| {
                let range = sentence.metadata().span().unwrap().byte_range();
                &source[range.start()..range.end()]
            })
            .collect();
        assert_eq!(represented, expected, "{source}");
        assert_eq!(network.reconstruct_text(), source);
        assert!(network.verify_full_match(None).is_clean(), "{source}");
    }
    record(&Observation {
        requirement_id: "I195-GRAMMAR-SENTENCE-ABBREVIATIONS",
        suffix: "behavior",
        fixture_id: "i195-grammar-sentence-abbreviations",
        fixture_file: "parity/fixtures/natural-language-sentence-abbreviations.lino",
        assertions: &[
            "sentenceSpansMatchExpectedText",
            "sourceReconstructsExactly",
            "parseHasNoRecoveryErrors",
        ],
        test_name: "sentence_abbreviations_preserve_context_and_true_boundaries",
    });
}

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
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
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
            assert_ne!(
                network.query_links(&LinkQuery::by_type(LinkType::Syntax).with_term("ERROR")),
                [] as [&meta_language::Link; 0]
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
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
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
