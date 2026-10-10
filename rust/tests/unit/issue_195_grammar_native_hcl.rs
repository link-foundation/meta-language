//! The generated HCL grammar is shared data; both runtimes execute it.
use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, oracle_rows, parse, parser, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};
use meta_language::{
    FeatureParseOptions, LinkNetwork, ParseConfiguration, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/hcl.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/hcl.json");
const CORPUS: &str = include_str!("../../../parity/fixtures/native-grammars/hcl-corpus.json");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-HCL",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-hcl",
        fixture_file: "parity/fixtures/native-grammars/hcl.json",
        assertions,
        test_name,
    });
}

#[test]
fn native_hcl_and_terraform_use_the_ordinary_catalog_parser() {
    let source = "a=<<END\nbody\nEND\n";
    for language in ["HCL", "terraform"] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        for term in ["config_file", "heredoc_template"] {
            assert!(
                network
                    .links()
                    .any(|link| link.metadata().term() == Some(term)),
                "{language}: {term}"
            );
        }
    }
    observe(
        &["nativeHclCatalogDispatch"],
        "native_hcl_and_terraform_use_the_ordinary_catalog_parser",
    );
}

#[test]
fn native_hcl_root_spans_include_leading_trivia() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    for source in ["\na=1\n", "  a=1\n", "\n\t\n"] {
        let tree = parse(&parser, source).unwrap();
        assert_eq!(
            rows.rows(&tree, source),
            oracle_rows(&tree_sitter_hcl::LANGUAGE.into(), source).0
        );
        for language in ["HCL", "terraform"] {
            let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
            assert_eq!(network.reconstruct_text(), source);
            let root = network
                .links()
                .find(|link| link.metadata().term() == Some("config_file"))
                .unwrap();
            assert_eq!(root.metadata().span().unwrap().byte_range().start(), 0);
        }
    }
}

#[test]
fn native_hcl_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).unwrap();
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("configuration_file"));
    observe(
        &["nativeHclGrammarIsCanonicalLinks"],
        "native_hcl_grammar_is_canonical_links_notation",
    );
}

#[test]
fn native_hcl_focused_trees_match_oracle_and_preserve_every_byte() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    oracle_agrees(&tree_sitter_hcl::LANGUAGE.into(), &fixture);
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("the native grammar accepts the source");
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
        assert_eq!(rebuilt(&tree), source(case));
    }
    observe(
        &["nativeHclTreesMatchOracle", "nativeHclTreesLossless"],
        "native_hcl_focused_trees_match_oracle_and_preserve_every_byte",
    );
}

#[test]
fn native_hcl_focused_invalid_sources_are_rejected_and_recovered_losslessly() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    for case in cases(&fixture, "rejections") {
        assert!(parse(&parser, source(case)).is_none());
        let outcome = parser
            .parse_tree(
                source(case).as_bytes(),
                &FeatureParseOptions {
                    error_recovery: Some(true),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap();
        let tree = outcome.tree.unwrap();
        assert!(!outcome.ok);
        assert_eq!(tree.render(), case["recovered"].as_str().unwrap());
        let recovered: String = leaves(&tree)
            .into_iter()
            .map(|leaf| match leaf {
                SyntaxTree::Token { text: leaf, .. } | SyntaxTree::Error { text: leaf, .. } => {
                    text(leaf)
                }
                SyntaxTree::Missing { start, end, .. } => {
                    assert_eq!(start, end);
                    ""
                }
                other => panic!("unexpected recovery leaf {other:?}"),
            })
            .collect();
        assert_eq!(recovered, source(case));
    }
    observe(
        &["nativeHclRejectsInvalidInput"],
        "native_hcl_focused_invalid_sources_are_rejected_and_recovered_losslessly",
    );
}

// CI executes the upstream corpus; local checks select the focused tests.
#[test]
fn native_hcl_upstream_corpus_matches_the_independent_oracle() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_hcl::LANGUAGE.into();
    let inputs = corpus["cases"].as_array().unwrap();
    assert_ne!(inputs.as_slice(), &[] as &[Value]);
    let mut failures = Vec::new();
    for case in inputs {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let (expected, recovers) = oracle_rows(&language, source(case));
            if recovers {
                assert!(parse(&parser, source(case)).is_none(), "{case}");
            } else {
                let tree = parse(&parser, source(case)).unwrap_or_else(|| panic!("{case}"));
                assert_eq!(rows.rows(&tree, source(case)), expected, "{case}");
                assert_eq!(rebuilt(&tree), source(case));
            }
        }));
        if let Err(error) = result {
            let message = error
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| error.downcast_ref::<&str>().copied())
                .unwrap_or("parse panicked");
            failures.push(format!("{}: {}: {message}", case["file"], case["title"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    observe(
        &["nativeHclUpstreamCorpusMatchesOracle"],
        "native_hcl_upstream_corpus_matches_the_independent_oracle",
    );
}
