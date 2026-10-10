//! The generated R grammar is shared data; both runtimes execute it.
use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, oracle_rows, parse, parser, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};
use meta_language::{
    FeatureParseOptions, LinkNetwork, ParseConfiguration, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/r.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/r.json");
const CORPUS: &str = include_str!("../../../parity/fixtures/native-grammars/r-corpus.json");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-R",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-r",
        fixture_file: "parity/fixtures/native-grammars/r.json",
        assertions,
        test_name,
    });
}

#[test]
fn native_r_and_lowercase_r_use_the_ordinary_catalog_parser() {
    let source = "x <- r\"(body)\"\n";
    for language in ["R", "r"] {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        for term in ["program", "string"] {
            assert!(
                network
                    .links()
                    .any(|link| link.metadata().term() == Some(term)),
                "{language}: {term}"
            );
        }
    }
    observe(
        &["nativeRCatalogDispatch"],
        "native_r_and_lowercase_r_use_the_ordinary_catalog_parser",
    );
}

#[test]
fn native_r_root_spans_include_leading_trivia() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    for source in ["\nx <- 1\n", "  x <- 1\n", "\n\t\n"] {
        let tree = parse(&parser, source).unwrap();
        assert_eq!(
            rows.rows(&tree, source),
            oracle_rows(&tree_sitter_r::LANGUAGE.into(), source).0
        );
        for language in ["R", "r"] {
            let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
            assert_eq!(network.reconstruct_text(), source);
            let root = network
                .links()
                .find(|link| link.metadata().term() == Some("program"))
                .unwrap();
            assert_eq!(root.metadata().span().unwrap().byte_range().start(), 0);
        }
    }
}

#[test]
fn native_r_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).unwrap();
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("program"));
    observe(
        &["nativeRGrammarIsCanonicalLinks"],
        "native_r_grammar_is_canonical_links_notation",
    );
}

#[test]
fn native_r_focused_trees_match_oracle_and_preserve_every_byte() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    oracle_agrees(&tree_sitter_r::LANGUAGE.into(), &fixture);
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
        &["nativeRTreesMatchOracle", "nativeRTreesLossless"],
        "native_r_focused_trees_match_oracle_and_preserve_every_byte",
    );
}

#[test]
fn native_r_focused_invalid_sources_are_rejected_and_recovered_losslessly() {
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
        &["nativeRRejectsInvalidInput"],
        "native_r_focused_invalid_sources_are_rejected_and_recovered_losslessly",
    );
}

// CI executes the upstream corpus; local checks select the focused tests.
#[test]
fn native_r_upstream_corpus_matches_the_independent_oracle() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_r::LANGUAGE.into();
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
        &["nativeRUpstreamCorpusMatchesOracle"],
        "native_r_upstream_corpus_matches_the_independent_oracle",
    );
}
