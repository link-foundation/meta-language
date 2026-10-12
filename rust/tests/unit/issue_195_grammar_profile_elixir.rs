//! Candidate Elixir grammar and quoted fragment and contextual scanner operations are shared Links data.
use super::issue_195_native_grammar_rows::{
    Rows, cases, oracle_agrees, oracle_rows, parse, parser, rebuilt, source,
};
use meta_language::{FeatureParseOptions, parse_grammar_links, render_grammar_links};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/elixir.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/elixir-profile.json");
const CORPUS: &str =
    include_str!("../../../parity/fixtures/native-grammars/elixir-profile-corpus.json");

#[test]
fn elixir_profile_focused_trees_match_the_independent_oracle() {
    assert_eq!(
        render_grammar_links(&parse_grammar_links(GRAMMAR).unwrap()),
        GRAMMAR
    );
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    oracle_agrees(&tree_sitter_elixir::LANGUAGE.into(), &fixture);
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("native accepts focused source");
        assert_eq!(Value::Array(rows.rows(&tree, source(case))), case["rows"]);
        assert_eq!(rebuilt(&tree), source(case));
    }
    for case in cases(&fixture, "rejections") {
        assert!(parse(&parser, source(case)).is_none());
        let native = parser
            .parse_tree(
                source(case).as_bytes(),
                &FeatureParseOptions {
                    error_recovery: Some(true),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap();
        assert_eq!(native.rejection.unwrap().reason, "recovered");
        assert_eq!(rebuilt(&native.tree.unwrap()), source(case));
    }
}

// CI executes the complete pinned corpus; local work uses focused JavaScript.
#[test]
fn elixir_profile_upstream_corpus_matches_the_independent_oracle() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_elixir::LANGUAGE.into();
    let inputs = corpus["cases"].as_array().unwrap();
    assert!(!inputs.is_empty());
    let mut failures = Vec::new();
    for case in inputs {
        let (expected, recovers) = oracle_rows(&language, source(case));
        let outcome = parser.parse_tree(source(case).as_bytes(), &FeatureParseOptions::default());
        let problem = match outcome {
            Err(error) => Some(format!("executor: {error:?}")),
            Ok(outcome) if recovers => outcome
                .ok
                .then(|| "native accepted invalid input".to_owned()),
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
}
