//! The generated HTML grammar is shared data; both runtimes execute it.
use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, oracle_rows, parse, parser, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};
use meta_language::{
    FeatureParseOptions, LinkNetwork, LinkType, ParseConfiguration, SyntaxTree,
    parse_grammar_links, render_grammar_links,
};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/html.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/html.json");
const CORPUS: &str = include_str!("../../../parity/fixtures/native-grammars/html-corpus.json");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-HTML",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-html",
        fixture_file: "parity/fixtures/native-grammars/html.json",
        assertions,
        test_name,
    });
}

#[test]
fn native_html_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).unwrap();
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    observe(
        &["nativeHtmlGrammarIsCanonicalLinks"],
        "native_html_grammar_is_canonical_links_notation",
    );
}

#[test]
fn native_html_focused_trees_match_oracle_and_preserve_every_byte() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    oracle_agrees(&tree_sitter_html::LANGUAGE.into(), &fixture);
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("the native grammar accepts the source");
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
        assert_eq!(rebuilt(&tree), source(case));
        let network = LinkNetwork::parse(
            source(case),
            fixture["language"].as_str().unwrap(),
            ParseConfiguration::default(),
        );
        assert_eq!(network.reconstruct_text(), source(case));
        assert!(
            network
                .parse_grammars()
                .iter()
                .any(|(_, grammar)| grammar.id == "native-html")
        );
    }
    observe(
        &["nativeHtmlTreesMatchOracle", "nativeHtmlTreesLossless"],
        "native_html_focused_trees_match_oracle_and_preserve_every_byte",
    );
}

#[test]
fn native_html_focused_invalid_sources_are_rejected_and_recovered_losslessly() {
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
        &["nativeHtmlRejectsInvalidInput"],
        "native_html_focused_invalid_sources_are_rejected_and_recovered_losslessly",
    );
}

// CI executes the upstream corpus; local checks select the focused tests.
#[test]
fn native_html_upstream_corpus_matches_the_independent_oracle() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_html::LANGUAGE.into();
    let inputs = corpus["cases"].as_array().unwrap();
    assert_ne!(inputs.as_slice(), &[] as &[Value]);
    let mut failures = Vec::new();
    for case in inputs {
        let (expected, recovers) = oracle_rows(&language, source(case));
        let outcome = parser.parse_tree(source(case).as_bytes(), &FeatureParseOptions::default());
        let problem = match outcome {
            Err(error) => Some(format!("executor: {error:?}")),
            Ok(outcome) if recovers => outcome
                .ok
                .then(|| "native accepted oracle-invalid input".to_owned()),
            Ok(outcome) if !outcome.ok => {
                Some(format!("native rejection: {:?}", outcome.rejection))
            }
            Ok(outcome) if !outcome.ambiguities.is_empty() => {
                Some("ambiguous native tree".to_owned())
            }
            Ok(outcome) => {
                let tree = outcome.tree.unwrap();
                if rows.rows(&tree, source(case)) != expected {
                    Some("concrete tree differs from oracle".to_owned())
                } else if rebuilt(&tree) != source(case) {
                    Some("source reconstruction differs".to_owned())
                } else {
                    None
                }
            }
        };
        if let Some(problem) = problem {
            failures.push(format!("{}: {}: {problem}", case["file"], case["title"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    observe(
        &["nativeHtmlUpstreamCorpusMatchesOracle"],
        "native_html_upstream_corpus_matches_the_independent_oracle",
    );
}

#[test]
fn native_html_focused_embedded_raw_text_preserves_leading_whitespace() {
    for (tag, language, body) in [
        ("script", "JavaScript", "\n  let leaf=7;\n"),
        ("style", "CSS", "\n  p {color:green}\n"),
    ] {
        let source = format!("<{tag}>{body}</{tag}>");
        let network = LinkNetwork::parse(&source, "HTML", ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        let mut embedded = network
            .links()
            .filter(|link| {
                link.metadata().link_type() == Some(LinkType::Token)
                    && link.metadata().language() == Some(language)
            })
            .filter_map(|link| {
                link.metadata().span().map(|span| {
                    (
                        span.byte_range().start(),
                        link.metadata().term().unwrap_or_default(),
                    )
                })
            })
            .collect::<Vec<_>>();
        embedded.sort_unstable();
        assert_eq!(
            embedded
                .into_iter()
                .map(|(_, term)| term)
                .collect::<String>(),
            body
        );
    }
}
